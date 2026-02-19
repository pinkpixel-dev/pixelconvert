use adw::subclass::prelude::*;
use gtk4::prelude::*;
use gtk4::{gdk, gio, glib};
use libadwaita as adw;

mod imp {
    use super::*;
    use adw::prelude::*;
    use std::cell::RefCell;

    #[derive(Debug)]
    pub struct PixelConvertWindow {
        pub toast_overlay: adw::ToastOverlay,
        pub content_stack: gtk4::Stack,
        pub status_page: adw::StatusPage,
        pub main_view: gtk4::Box,
        pub selected_files: RefCell<Vec<std::path::PathBuf>>,
        pub format_dropdown: gtk4::DropDown,
        pub quality_scale: gtk4::Scale,
        pub convert_button: gtk4::Button,
        pub file_list: gtk4::ListBox,
        pub progress_bar: gtk4::ProgressBar,
        pub status_label: gtk4::Label,
        pub is_converting: RefCell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PixelConvertWindow {
        const NAME: &'static str = "PixelConvertWindow";
        type Type = super::PixelConvertWindow;
        type ParentType = adw::ApplicationWindow;

        fn new() -> Self {
            Self {
                toast_overlay: adw::ToastOverlay::new(),
                content_stack: gtk4::Stack::new(),
                status_page: adw::StatusPage::new(),
                main_view: gtk4::Box::new(gtk4::Orientation::Vertical, 0),
                selected_files: RefCell::new(Vec::new()),
                format_dropdown: gtk4::DropDown::from_strings(&[]),
                quality_scale: gtk4::Scale::with_range(
                    gtk4::Orientation::Horizontal,
                    0.0,
                    100.0,
                    1.0,
                ),
                convert_button: gtk4::Button::new(),
                file_list: gtk4::ListBox::new(),
                progress_bar: gtk4::ProgressBar::new(),
                status_label: gtk4::Label::new(None),
                is_converting: RefCell::new(false),
            }
        }
    }

    impl ObjectImpl for PixelConvertWindow {
        fn constructed(&self) {
            self.parent_constructed();

            let obj = self.obj();

            // Set up the window
            obj.set_title(Some("PixelConvert"));
            obj.set_default_size(900, 650);

            // Create empty state status page
            const LOGO_BYTES: &[u8] =
                include_bytes!("../data/icons/org.pinkpixel.PixelConvert.png");
            let logo_glib_bytes = glib::Bytes::from_static(LOGO_BYTES);
            if let Ok(texture) = gdk::Texture::from_bytes(&logo_glib_bytes) {
                self.status_page.set_paintable(Some(&texture));
            } else {
                self.status_page
                    .set_icon_name(Some("image-x-generic-symbolic"));
            }
            self.status_page.set_title("Welcome to PixelConvert");
            self.status_page
                .set_description(Some("Drag and drop images here or click to select files"));
            self.status_page.set_vexpand(true);

            // Add "Select Files" button to status page
            let select_button = gtk4::Button::with_label("Select Files");
            select_button.set_css_classes(&["pill", "suggested-action"]);
            select_button.connect_clicked(glib::clone!(
                #[weak]
                obj,
                move |_| {
                    obj.imp().open_file_chooser();
                }
            ));
            self.status_page.set_child(Some(&select_button));

            // Create main view with controls
            self.main_view.set_orientation(gtk4::Orientation::Vertical);
            self.main_view.set_spacing(12);
            self.main_view.set_margin_top(12);
            self.main_view.set_margin_bottom(12);
            self.main_view.set_margin_start(12);
            self.main_view.set_margin_end(12);

            // Header with file selection button
            let header_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 12);
            let add_files_button = gtk4::Button::with_label("Add Files");
            add_files_button.set_icon_name("list-add-symbolic");
            add_files_button.connect_clicked(glib::clone!(
                #[weak]
                obj,
                move |_| {
                    obj.imp().open_file_chooser();
                }
            ));
            header_box.append(&add_files_button);

            let clear_button = gtk4::Button::with_label("Clear");
            clear_button.set_icon_name("user-trash-symbolic");
            clear_button.connect_clicked(glib::clone!(
                #[weak]
                obj,
                move |_| {
                    obj.imp().clear_files();
                }
            ));
            header_box.append(&clear_button);

            self.main_view.append(&header_box);

            // File list in scrolled window
            let scrolled = gtk4::ScrolledWindow::builder()
                .vexpand(true)
                .hexpand(true)
                .min_content_height(200)
                .build();

            self.file_list.set_css_classes(&["boxed-list"]);
            self.file_list.set_selection_mode(gtk4::SelectionMode::None);
            scrolled.set_child(Some(&self.file_list));
            self.main_view.append(&scrolled);

            // Conversion controls
            let controls_group = adw::PreferencesGroup::new();
            controls_group.set_title("Conversion Settings");

            // Format selection
            let format_row = adw::ActionRow::new();
            format_row.set_title("Output Format");

            let formats = gtk4::StringList::new(&[
                "PNG", "JPEG", "WebP", "AVIF", "GIF", "BMP", "TIFF", "ICO",
            ]);
            self.format_dropdown.set_model(Some(&formats));
            self.format_dropdown.set_selected(2); // Default to WebP
            self.format_dropdown.set_valign(gtk4::Align::Center);
            format_row.add_suffix(&self.format_dropdown);
            controls_group.add(&format_row);

            // Quality slider
            let quality_row = adw::ActionRow::new();
            quality_row.set_title("Quality");
            quality_row.set_subtitle("Higher quality = larger file size");

            self.quality_scale.set_value(85.0);
            self.quality_scale.set_draw_value(true);
            self.quality_scale.set_value_pos(gtk4::PositionType::Right);
            self.quality_scale.set_hexpand(true);
            self.quality_scale.set_width_request(200);
            quality_row.add_suffix(&self.quality_scale);
            controls_group.add(&quality_row);

            self.main_view.append(&controls_group);

            // Convert button
            self.convert_button.set_label("Convert Images");
            self.convert_button
                .set_css_classes(&["suggested-action", "pill"]);
            self.convert_button.set_halign(gtk4::Align::Center);
            self.convert_button.set_size_request(200, -1);
            self.convert_button.connect_clicked(glib::clone!(
                #[weak]
                obj,
                move |_| {
                    obj.imp().start_conversion();
                }
            ));

            let button_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
            button_box.set_halign(gtk4::Align::Center);
            button_box.set_margin_top(12);
            button_box.append(&self.convert_button);
            self.main_view.append(&button_box);

            // Progress bar
            self.progress_bar.set_visible(false);
            self.progress_bar.set_margin_top(12);
            self.progress_bar.set_margin_start(24);
            self.progress_bar.set_margin_end(24);
            self.main_view.append(&self.progress_bar);

            // Status label
            self.status_label.set_visible(false);
            self.status_label.set_margin_top(6);
            self.status_label.set_css_classes(&["dim-label", "caption"]);
            self.status_label.set_halign(gtk4::Align::Center);
            self.main_view.append(&self.status_label);

            // Stack for switching between empty and main view
            self.content_stack
                .add_named(&self.status_page, Some("empty"));
            self.content_stack.add_named(&self.main_view, Some("main"));
            self.content_stack.set_visible_child_name("empty");

            // Wrap in toast overlay and set as window content
            self.toast_overlay.set_child(Some(&self.content_stack));

            // Create header bar with window controls and menu
            let header_bar = adw::HeaderBar::new();

            // Add a menu button with app actions
            let menu = gio::Menu::new();
            menu.append(Some("_Keyboard Shortcuts"), Some("app.shortcuts"));
            menu.append(Some("_About PixelConvert"), Some("app.about"));
            menu.append(Some("_Quit"), Some("app.quit"));

            let menu_button = gtk4::MenuButton::new();
            menu_button.set_icon_name("open-menu-symbolic");
            menu_button.set_menu_model(Some(&menu));
            menu_button.set_tooltip_text(Some("Main Menu"));
            header_bar.pack_end(&menu_button);

            // Assemble: header bar + toast overlay in a vertical box
            let outer_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
            outer_box.append(&header_bar);
            outer_box.append(&self.toast_overlay);

            obj.set_content(Some(&outer_box));

            // Set up drag-and-drop
            self.setup_drag_drop(&obj);
        }
    }

    impl WidgetImpl for PixelConvertWindow {}
    impl WindowImpl for PixelConvertWindow {}
    impl ApplicationWindowImpl for PixelConvertWindow {}
    impl AdwApplicationWindowImpl for PixelConvertWindow {}

    impl PixelConvertWindow {
        fn setup_drag_drop(&self, window: &super::PixelConvertWindow) {
            let drop_target =
                gtk4::DropTarget::new(gio::File::static_type(), gtk4::gdk::DragAction::COPY);

            drop_target.connect_drop(glib::clone!(
                #[weak]
                window,
                #[upgrade_or]
                false,
                move |_, value, _, _| {
                    if let Ok(file) = value.get::<gio::File>() {
                        if let Some(path) = file.path() {
                            window.imp().add_file(path);
                            return true;
                        }
                    }
                    false
                }
            ));

            window.add_controller(drop_target);
        }

        pub fn open_file_chooser(&self) {
            let window = self.obj();
            let window_ref = window.upcast_ref::<gtk4::Window>();

            let filter = gtk4::FileFilter::new();
            filter.set_name(Some("Image Files"));
            // Add individual MIME types for broader compatibility
            filter.add_mime_type("image/png");
            filter.add_mime_type("image/jpeg");
            filter.add_mime_type("image/webp");
            filter.add_mime_type("image/avif");
            filter.add_mime_type("image/gif");
            filter.add_mime_type("image/bmp");
            filter.add_mime_type("image/tiff");
            filter.add_mime_type("image/x-icon");
            // Also add glob patterns as fallback
            filter.add_suffix("png");
            filter.add_suffix("jpg");
            filter.add_suffix("jpeg");
            filter.add_suffix("webp");
            filter.add_suffix("avif");
            filter.add_suffix("gif");
            filter.add_suffix("bmp");
            filter.add_suffix("tiff");
            filter.add_suffix("tif");
            filter.add_suffix("ico");

            // Also add an "All Files" filter
            let all_filter = gtk4::FileFilter::new();
            all_filter.set_name(Some("All Files"));
            all_filter.add_pattern("*");

            let filters = gio::ListStore::new::<gtk4::FileFilter>();
            filters.append(&filter);
            filters.append(&all_filter);

            let dialog = gtk4::FileDialog::builder()
                .title("Select Images")
                .modal(true)
                .filters(&filters)
                .default_filter(&filter)
                .build();

            dialog.open_multiple(
                Some(window_ref),
                gio::Cancellable::NONE,
                glib::clone!(
                    #[weak(rename_to = imp)]
                    self,
                    move |result| {
                        if let Ok(files) = result {
                            for i in 0..files.n_items() {
                                if let Some(file) = files.item(i).and_downcast::<gio::File>() {
                                    if let Some(path) = file.path() {
                                        imp.add_file(path);
                                    }
                                }
                            }
                        }
                    }
                ),
            );
        }

        fn add_file(&self, path: std::path::PathBuf) {
            // Check if it's an image file
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                let valid_extensions = [
                    "png", "jpg", "jpeg", "gif", "webp", "avif", "bmp", "tiff", "ico",
                ];

                if !valid_extensions.contains(&ext_str.as_str()) {
                    return;
                }
            } else {
                return;
            }

            // Add to list if not already there
            let mut files = self.selected_files.borrow_mut();
            if !files.contains(&path) {
                files.push(path.clone());

                // Create row for file
                let row = adw::ActionRow::new();
                row.set_title(&path.file_name().unwrap().to_string_lossy());
                row.set_subtitle(
                    &path
                        .parent()
                        .unwrap_or(std::path::Path::new(""))
                        .to_string_lossy(),
                );

                let remove_button = gtk4::Button::from_icon_name("user-trash-symbolic");
                remove_button.set_valign(gtk4::Align::Center);
                remove_button.set_css_classes(&["flat", "circular"]);

                let path_clone = path.clone();
                remove_button.connect_clicked(glib::clone!(
                    #[weak(rename_to = imp)]
                    self,
                    move |_| {
                        imp.remove_file(&path_clone);
                    }
                ));

                row.add_suffix(&remove_button);
                self.file_list.append(&row);
            }

            // Show main view if we have files
            if !files.is_empty() {
                self.content_stack.set_visible_child_name("main");
            }
        }

        fn remove_file(&self, path: &std::path::Path) {
            let mut files = self.selected_files.borrow_mut();
            if let Some(pos) = files.iter().position(|p| p == path) {
                files.remove(pos);

                // Remove from UI
                if let Some(row) = self.file_list.row_at_index(pos as i32) {
                    self.file_list.remove(&row);
                }
            }

            // Switch back to empty view if no files
            if files.is_empty() {
                self.content_stack.set_visible_child_name("empty");
            }
        }

        pub fn clear_files(&self) {
            self.selected_files.borrow_mut().clear();

            // Remove all rows
            while let Some(row) = self.file_list.row_at_index(0) {
                self.file_list.remove(&row);
            }

            self.content_stack.set_visible_child_name("empty");
        }

        pub fn start_conversion(&self) {
            // Check if already converting
            if *self.is_converting.borrow() {
                return;
            }

            let files = self.selected_files.borrow().clone();
            if files.is_empty() {
                return;
            }

            // Get conversion settings
            let format_idx = self.format_dropdown.selected();
            let quality = self.quality_scale.value() as u8;

            // Map format index to SupportedFormat
            use crate::converter::SupportedFormat;
            let format = match format_idx {
                0 => SupportedFormat::Png,
                1 => SupportedFormat::Jpeg,
                2 => SupportedFormat::WebP,
                3 => SupportedFormat::Avif,
                4 => SupportedFormat::Gif,
                5 => SupportedFormat::Bmp,
                6 => SupportedFormat::Tiff,
                7 => SupportedFormat::Ico,
                _ => SupportedFormat::WebP,
            };

            // Mark as converting and disable button
            *self.is_converting.borrow_mut() = true;
            self.convert_button.set_sensitive(false);
            self.progress_bar.set_visible(true);
            self.progress_bar.set_fraction(0.0);
            self.status_label.set_visible(true);
            self.status_label.set_text("Starting conversion...");

            // Build batch jobs
            use crate::batch::BatchJob;
            use crate::converter::ConversionOptions;

            let jobs: Vec<BatchJob> = files
                .iter()
                .map(|path| BatchJob {
                    input_path: path.clone(),
                    output_path: path.with_extension(format.extension()),
                    options: ConversionOptions { quality, format },
                })
                .collect();

            let total = jobs.len();

            // Create a std::sync::mpsc channel for thread-safe progress
            let (sender, receiver) = std::sync::mpsc::channel::<crate::batch::BatchProgress>();

            // Poll the receiver from the GLib main loop
            let window = self.obj().clone();
            let completed = std::rc::Rc::new(std::cell::Cell::new(0usize));
            let failed = std::rc::Rc::new(std::cell::Cell::new(0usize));

            glib::timeout_add_local(
                std::time::Duration::from_millis(50),
                glib::clone!(
                    #[weak]
                    window,
                    #[strong]
                    completed,
                    #[strong]
                    failed,
                    #[upgrade_or]
                    glib::ControlFlow::Break,
                    move || {
                        use crate::batch::BatchProgress;
                        let imp = window.imp();

                        // Drain all available messages
                        while let Ok(progress) = receiver.try_recv() {
                            match progress {
                                BatchProgress::Processing { file } => {
                                    imp.status_label
                                        .set_text(&format!("Converting {}...", file));
                                }
                                BatchProgress::Completed { .. } => {
                                    completed.set(completed.get() + 1);
                                    imp.progress_bar
                                        .set_fraction(completed.get() as f64 / total as f64);
                                }
                                BatchProgress::Failed { file, error, .. } => {
                                    failed.set(failed.get() + 1);
                                    completed.set(completed.get() + 1);
                                    eprintln!("Failed to convert {}: {}", file, error);
                                    imp.progress_bar
                                        .set_fraction(completed.get() as f64 / total as f64);
                                }
                                BatchProgress::Finished { successful, failed } => {
                                    // Reset UI state
                                    *imp.is_converting.borrow_mut() = false;
                                    imp.convert_button.set_sensitive(true);
                                    imp.status_label.set_text(&format!(
                                        "Completed: {} succeeded, {} failed",
                                        successful, failed
                                    ));

                                    // Show completion toast
                                    let toast = if failed == 0 {
                                        adw::Toast::new(&format!(
                                            "Successfully converted {} images",
                                            successful
                                        ))
                                    } else {
                                        adw::Toast::new(&format!(
                                            "Converted {} images ({} failed)",
                                            successful, failed
                                        ))
                                    };
                                    toast.set_timeout(5);
                                    imp.toast_overlay.add_toast(toast);

                                    // Hide progress after a delay
                                    glib::timeout_add_seconds_local_once(
                                        3,
                                        glib::clone!(
                                            #[weak]
                                            window,
                                            move || {
                                                let imp = window.imp();
                                                imp.progress_bar.set_visible(false);
                                                imp.status_label.set_visible(false);
                                            }
                                        ),
                                    );

                                    return glib::ControlFlow::Break;
                                }
                            }
                        }
                        glib::ControlFlow::Continue
                    }
                ),
            );

            // Kick off conversion on background threads
            crate::batch::run_batch(jobs, sender);
        }
    }
}

glib::wrapper! {
    pub struct PixelConvertWindow(ObjectSubclass<imp::PixelConvertWindow>)
        @extends gtk4::Widget, gtk4::Window, gtk4::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap, gtk4::Accessible, gtk4::Buildable,
                    gtk4::ConstraintTarget, gtk4::Native, gtk4::Root, gtk4::ShortcutManager;
}

impl PixelConvertWindow {
    pub fn new(app: &adw::Application) -> Self {
        glib::Object::builder().property("application", app).build()
    }

    pub fn open_files(&self) {
        self.imp().open_file_chooser();
    }

    pub fn convert(&self) {
        self.imp().start_conversion();
    }

    pub fn clear(&self) {
        self.imp().clear_files();
    }
}
