use crate::converter::{ConversionOptions, ImageConverter};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// Progress update from batch processor
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum BatchProgress {
    Processing { file: String },
    Completed { file: String },
    Failed { file: String, error: String },
    Finished { successful: usize, failed: usize },
}

/// Batch conversion job
#[derive(Clone)]
pub struct BatchJob {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub options: ConversionOptions,
}

/// Run batch conversion on a background thread pool, sending progress
/// back via a `std::sync::mpsc::Sender` (glib receiver handles the UI side).
pub fn run_batch(jobs: Vec<BatchJob>, sender: std::sync::mpsc::Sender<BatchProgress>) {
    std::thread::spawn(move || {
        let successful = Arc::new(Mutex::new(0usize));
        let failed = Arc::new(Mutex::new(0usize));
        let max_concurrent = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4);
        let semaphore = Arc::new(Semaphore::new(max_concurrent));

        std::thread::scope(|scope| {
            for job in &jobs {
                semaphore.acquire();
                let sem = Arc::clone(&semaphore);
                let sender = sender.clone();
                let successful = Arc::clone(&successful);
                let failed = Arc::clone(&failed);

                let job = job.clone();
                scope.spawn(move || {
                    let file_name = job
                        .input_path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown")
                        .to_string();

                    let _ = sender.send(BatchProgress::Processing {
                        file: file_name.clone(),
                    });

                    let converter = ImageConverter::new(job.options.clone());
                    match converter.convert(&job.input_path, &job.output_path) {
                        Ok(_) => {
                            *successful.lock().unwrap() += 1;
                            let _ = sender.send(BatchProgress::Completed { file: file_name });
                        }
                        Err(e) => {
                            *failed.lock().unwrap() += 1;
                            let _ = sender.send(BatchProgress::Failed {
                                file: file_name,
                                error: e.to_string(),
                            });
                        }
                    }

                    sem.release();
                });
            }
        });

        let s = *successful.lock().unwrap();
        let f = *failed.lock().unwrap();
        let _ = sender.send(BatchProgress::Finished {
            successful: s,
            failed: f,
        });
    });
}

/// Simple counting semaphore using std primitives
struct Semaphore {
    count: Mutex<usize>,
    condvar: std::sync::Condvar,
}

impl Semaphore {
    fn new(permits: usize) -> Self {
        Self {
            count: Mutex::new(permits),
            condvar: std::sync::Condvar::new(),
        }
    }

    fn acquire(&self) {
        let mut count = self.count.lock().unwrap();
        while *count == 0 {
            count = self.condvar.wait(count).unwrap();
        }
        *count -= 1;
    }

    fn release(&self) {
        let mut count = self.count.lock().unwrap();
        *count += 1;
        self.condvar.notify_one();
    }
}
