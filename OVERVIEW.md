# PixelConvert — Technical Overview

This document provides a technical overview of the PixelConvert project for developers looking to understand, modify, or extend the codebase.

## Architecture

PixelConvert is a GTK4/Libadwaita desktop application written in Rust. It converts images between modern and traditional formats with batch processing support.

```
┌──────────────────────────────────────────────────┐
│                    main.rs                       │
│         Application bootstrap & actions          │
└──────────────┬───────────────────────────────────┘
               │
┌──────────────▼───────────────────────────────────┐
│                   window.rs                      │
│    GTK4 ObjectSubclass window (UI + state)       │
│  ┌─────────────┐  ┌───────────┐  ┌───────────┐  │
│  │ File list   │  │ Controls  │  │ Progress  │  │
│  │ (ListBox)   │  │ (format,  │  │ (bar +    │  │
│  │             │  │  quality) │  │  status)  │  │
│  └─────────────┘  └───────────┘  └───────────┘  │
└──────┬───────────────────┬───────────────────────┘
       │                   │
┌──────▼──────┐     ┌──────▼──────┐
│ converter.rs│     │  batch.rs   │
│ Single-file │◄────│ Parallel    │
│ conversion  │     │ thread pool │
└─────────────┘     └─────────────┘
```

## Source Files

### `src/main.rs` — Application Entry Point

- Creates the `adw::Application` with ID `dev.pinkpixel.PixelConvert`
- Registers keyboard shortcuts (`Ctrl+O`, `Ctrl+Enter`, `Ctrl+Q`, etc.)
- Sets up application-level actions: `quit`, `about`, `shortcuts`
- Sets up window-level actions: `open`, `convert`, `clear`
- Creates the `AboutDialog` and `ShortcutsWindow`

### `src/window.rs` — Main Window

Uses the GTK4 `ObjectSubclass` pattern to define `PixelConvertWindow`:

**Structure (`mod imp`)**:

- `PixelConvertWindow` struct holds all UI widgets as fields
- `ObjectSubclass` impl with `ParentType = adw::ApplicationWindow`
- `ObjectImpl::constructed()` builds the entire UI programmatically
- Required trait impls: `WidgetImpl`, `WindowImpl`, `ApplicationWindowImpl`, `AdwApplicationWindowImpl`

**UI Layout**:

- `adw::HeaderBar` — title bar with minimize/maximize/close + hamburger menu
- `adw::ToastOverlay` — wraps content for toast notifications
- `gtk4::Stack` — switches between empty state (`adw::StatusPage`) and main view
- Main view: file list (`gtk4::ListBox`), format dropdown (`gtk4::DropDown`), quality slider (`gtk4::Scale`), output directory picker, convert button, progress bar, status label

**Key Methods** (on `imp::PixelConvertWindow`):

- `open_file_chooser()` — opens `gtk4::FileDialog` with image MIME/suffix filters
- `pick_output_dir()` — opens `gtk4::FileDialog::select_folder()`, stores chosen path
- `add_file(path)` — validates extension, adds to list and UI
- `remove_file(path)` / `clear_files()` — file management
- `start_conversion()` — builds `BatchJob`s (respecting output dir), creates `mpsc` channel, starts polling timer, kicks off `batch::run_batch()`

**Public wrapper** (`PixelConvertWindow`):

- `new(app)`, `open_files()`, `convert()`, `clear()` — delegate to `imp`

### `src/converter.rs` — Image Conversion Engine

- `SupportedFormat` enum: `Png`, `Jpeg`, `WebP`, `Avif`, `Gif`, `Bmp`, `Tiff`, `Ico`
  - Methods: `extension()`, `mime_type()`, `display_name()`, `all()`
- `ConversionOptions`: `quality: u8` (0-100) + `format: SupportedFormat`
- `ImageConverter`: stateful converter holding options
  - `load_image(path)` — uses `image::open()` (supports all enabled codecs)
  - `convert(input, output)` — load + save pipeline
  - `save_image()` — dispatches to format-specific savers
  - `save_webp()` — uses `webp::Encoder::from_image().encode(quality)`
  - `save_avif()` — converts to RGBA8 pixels, uses `ravif::Encoder` with quality/speed settings

### `src/batch.rs` — Parallel Batch Processor

Designed to run outside the GLib main loop using OS threads:

- `BatchProgress` enum: `Processing`, `Completed`, `Failed`, `Finished` — sent via `mpsc`
- `BatchJob`: input path + output path + conversion options
- `run_batch(jobs, sender)` — spawns a background thread that:
  1. Creates a custom `Semaphore` (based on `Mutex` + `Condvar`) to limit concurrency
  2. Uses `std::thread::scope` for parallel execution
  3. Each thread acquires a permit, runs `ImageConverter::convert()`, sends progress via `mpsc::Sender`
  4. After all threads complete, sends `Finished` with success/failure counts

**Why not Tokio?** GTK4 applications use the GLib main loop. Tokio's reactor requires its own runtime, and `tokio::spawn` panics without one. The `std::thread` approach is simpler and avoids the runtime conflict entirely.

**UI Integration**: `window.rs` creates a `std::sync::mpsc::channel`, passes the sender to `run_batch()`, and polls the receiver every 50ms via `glib::timeout_add_local` to update progress bar and status on the main thread.

### `src/preferences.rs` — Preferences Window (Stub)

Defines `PreferencesWindow` using `adw::PreferencesWindow` with:

- Appearance group (dark mode toggle)
- Conversion defaults group (quality, format, concurrent threads)
- Output group (naming pattern, overwrite toggle)

Currently not wired into the UI — reserved for v1.1.

### `src/preview.rs` — Image Preview (Stub)

Defines `PreviewWidget` with split-pane before/after comparison using `gtk4::Picture`.
Currently not wired into the UI — reserved for v1.1.

## Dependencies

| Crate        | Version     | Purpose                                                              |
| ------------ | ----------- | -------------------------------------------------------------------- |
| `gtk4`       | 0.9 (v4_12) | UI framework                                                         |
| `libadwaita` | 0.7 (v1_5)  | GNOME design patterns, adaptive layouts                              |
| `glib`       | 0.20        | GLib bindings (main loop, signals, object system)                    |
| `gio`        | 0.20        | GIO bindings (file I/O, actions, menus)                              |
| `image`      | 0.25        | Image decoding/encoding (PNG, JPEG, GIF, BMP, TIFF, ICO, WebP, AVIF) |
| `webp`       | 0.3         | High-quality WebP encoding via libwebp                               |
| `ravif`      | 0.11        | AVIF encoding via rav1e                                              |
| `rgb`        | 0.8         | Pixel type conversions for ravif                                     |
| `anyhow`     | 1.0         | Error handling with context                                          |
| `thiserror`  | 1.0         | Derive macro for custom error types                                  |
| `once_cell`  | 1.19        | Lazy static initialization                                           |

## Build System

### Cargo (Development)

```bash
cargo build            # Debug build
cargo build --release  # Optimized release build (LTO, stripped)
cargo run              # Build and run
cargo check            # Type-check without codegen
cargo test             # Run unit tests
```

**System dependencies** (must be installed):

- GTK4 development libraries (`gtk4-devel` / `libgtk-4-dev`)
- Libadwaita development libraries (`libadwaita-1-dev`)
- NASM assembler (for rav1e/AVIF compilation)
- pkg-config

### Meson (Flatpak)

The project uses a Meson + Cargo hybrid build for Flatpak packaging:

- `meson.build` — top-level build config, finds GNOME dependencies
- `src/meson.build` — invokes Cargo to build the Rust binary
- `build-aux/meson/postinstall.py` — compiles GSettings schemas, updates icon cache

### Flatpak

```bash
# Build and install locally
flatpak-builder --user --install --force-clean build-dir dev.pinkpixel.PixelConvert.yml

# Run
flatpak run dev.pinkpixel.PixelConvert
```

The manifest (`dev.pinkpixel.PixelConvert.yml`) targets GNOME 49 runtime and bundles:

- libwebp 1.4.0
- libheif 1.18.2
- dav1d 1.4.3
- rav1e 0.7.1

## Data Files

| File                                              | Purpose                                     |
| ------------------------------------------------- | ------------------------------------------- |
| `data/dev.pinkpixel.PixelConvert.desktop.in`      | Desktop entry for app launchers             |
| `data/dev.pinkpixel.PixelConvert.metainfo.xml.in` | AppStream metadata for software centers     |
| `data/dev.pinkpixel.PixelConvert.gschema.xml`     | GSettings schema for persistent preferences |
| `data/icons/dev.pinkpixel.PixelConvert.png`       | Application icon                            |

## Threading Model

```
┌───────────────────────────────┐
│       GLib Main Loop          │
│  (UI updates, event handling) │
│                               │
│  polls mpsc::Receiver every   │
│  50ms via timeout_add_local   │
└───────────┬───────────────────┘
            │ std::sync::mpsc
┌───────────▼───────────────────┐
│    Background Thread          │
│  (spawned by run_batch)       │
│                               │
│  ┌─────────────────────────┐  │
│  │   std::thread::scope    │  │
│  │                         │  │
│  │  Worker 1  Worker 2 ... │  │
│  │  (limited by Semaphore) │  │
│  └─────────────────────────┘  │
└───────────────────────────────┘
```

- The GLib main loop is single-threaded and handles all UI operations
- Image conversion runs on background OS threads via `std::thread::scope`
- A custom `Semaphore` (Mutex + Condvar) limits concurrency to available CPU cores
- Progress messages flow from worker threads → `mpsc::Sender` → main loop polls `mpsc::Receiver`

## Key Design Decisions

1. **No Tokio runtime**: GTK4 uses the GLib main loop; Tokio would conflict. All async work is done with OS threads and `mpsc` channels.

2. **Programmatic UI**: The UI is built in Rust code rather than XML Blueprint/UI files. This keeps everything in one language and avoids build-time resource compilation during development.

3. **ObjectSubclass pattern**: The standard GTK4-rs approach for creating custom widgets. The `imp` module holds the internal state, and the outer type is a GLib object wrapper.

4. **Separate encoding crates**: While `image` handles most formats, WebP and AVIF encoding use dedicated crates (`webp`, `ravif`) for better quality and control over encoding parameters.

5. **Output path convention**: Converted files are saved alongside the originals with the new extension by default (e.g., `photo.jpg` → `photo.webp`). A custom output directory can be selected via the "Output Directory" row in Conversion Settings.

## Extending the Project

### Adding a New Format

1. Add variant to `SupportedFormat` in `converter.rs`
2. Implement `extension()`, `mime_type()`, `display_name()` matches
3. Add to `all()` vector
4. Add save logic in `save_image()` (and a dedicated `save_xyz()` if needed)
5. Add to format dropdown in `window.rs` `ObjectImpl::constructed()`
6. Add MIME type and suffix to file chooser filter in `open_file_chooser()`
7. Add extension to `add_file()` validation list
8. Enable the feature in `Cargo.toml` `image` crate if applicable
9. Update the format index mapping in `start_conversion()`

### Adding Preferences Persistence

1. Wire `preferences.rs` `PreferencesWindow` to the hamburger menu
2. Use `gio::Settings` with the existing GSettings schema to load/save values
3. Apply saved defaults to format dropdown and quality slider on window construction

### Adding Image Preview

1. Expand `preview.rs` `PreviewWidget` with actual image loading
2. Add the widget to the main view in `window.rs`
3. Connect file list selection changes to update the preview
