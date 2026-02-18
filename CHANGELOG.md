# Changelog

All notable changes to PixelConvert will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Header bar with minimize, maximize, and close window controls
- Hamburger menu with Keyboard Shortcuts, About, and Quit actions
- WebP and AVIF decoding support (input files)
- Individual MIME type and file extension filters in the file chooser
- "All Files" fallback filter in file chooser dialog
- `glib::timeout_add_local` polling for thread-safe UI progress updates

### Changed

- Replaced Tokio async runtime with `std::thread::scope` and `std::sync::mpsc` for batch processing — GTK4 apps run on the GLib main loop, not a Tokio reactor
- Removed `tokio` dependency entirely, reducing binary size and compile time
- File chooser now lists each supported MIME type individually instead of `image/*` wildcard for better desktop compatibility

### Fixed

- **File chooser showing no images**: `image/*` MIME wildcard was not recognized by `FileDialog` on some systems; replaced with individual MIME types and suffix patterns
- **"Successfully converted 0 images"**: Batch processor panicked with "there is no reactor running" because `tokio::spawn` requires a Tokio runtime; rewrote to use `std::thread` with a custom counting semaphore
- **WebP/AVIF files failing to open**: `image` crate had `default-features = false` without `webp` and `avif` features, so it couldn't decode those input formats
- Silenced dead code warnings for modules reserved for future use (preferences, preview)

## [1.0.0] - TBD

### Added

- Modern GTK4 + Libadwaita user interface
- Support for 8 image formats: PNG, JPEG, WebP, AVIF, GIF, BMP, TIFF, ICO
- Batch conversion with parallel processing
- Real-time progress tracking with progress bar and status label
- Drag-and-drop file selection
- Quality adjustment slider (0-100)
- Keyboard shortcuts (Ctrl+O, Ctrl+Enter, Ctrl+Shift+Delete, Ctrl+Q, Ctrl+?, Ctrl+,)
- About dialog with project information
- Shortcuts window for keyboard reference
- Automatic dark/light theme following system preference
- Toast notifications for conversion results

### Technical

- Built with Rust 2021 edition
- GTK4 4.12+ and Libadwaita 1.5+ for modern GNOME integration
- Parallel batch processing via `std::thread::scope` with counting semaphore
- WebP encoding via `webp` crate, decoding via `image` crate
- AVIF encoding via `ravif` and `rav1e`, decoding via `image` crate
- Custom counting semaphore (`Mutex` + `Condvar`) for thread pool limiting
- Meson build system integration for Flatpak packaging
- Flatpak manifest targeting GNOME 47 runtime

[Unreleased]: https://github.com/pinkpixel-dev/pixelconvert/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/pinkpixel-dev/pixelconvert/releases/tag/v1.0.0
