use gtk4::prelude::*;

/// Preview widget for showing before/after image comparison
#[allow(dead_code)]
pub struct PreviewWidget {
    container: gtk4::Box,
    original_view: gtk4::Picture,
    preview_view: gtk4::Picture,
}

#[allow(dead_code)]
impl PreviewWidget {
    pub fn new() -> Self {
        let container = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Horizontal)
            .spacing(12)
            .homogeneous(true)
            .build();

        // Original image view
        let original_box = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .spacing(6)
            .build();

        let original_label = gtk4::Label::builder()
            .label("Original")
            .css_classes(vec!["title-4"])
            .build();

        let original_view = gtk4::Picture::builder().can_shrink(true).build();

        original_box.append(&original_label);
        original_box.append(&original_view);

        // Preview image view
        let preview_box = gtk4::Box::builder()
            .orientation(gtk4::Orientation::Vertical)
            .spacing(6)
            .build();

        let preview_label = gtk4::Label::builder()
            .label("Preview")
            .css_classes(vec!["title-4"])
            .build();

        let preview_view = gtk4::Picture::builder().can_shrink(true).build();

        preview_box.append(&preview_label);
        preview_box.append(&preview_view);

        container.append(&original_box);
        container.append(&preview_box);

        Self {
            container,
            original_view,
            preview_view,
        }
    }

    pub fn widget(&self) -> &gtk4::Box {
        &self.container
    }

    pub fn set_original_from_file(&self, path: &std::path::Path) {
        if let Some(file) = gtk4::gio::File::for_path(path).path() {
            self.original_view.set_filename(Some(&file));
        }
    }

    pub fn clear(&self) {
        self.original_view.set_paintable(gtk4::gdk::Paintable::NONE);
        self.preview_view.set_paintable(gtk4::gdk::Paintable::NONE);
    }

    // TODO: Implement actual preview generation
    // TODO: Add zoom/pan controls
    // TODO: Show image info (dimensions, size, format)
}
