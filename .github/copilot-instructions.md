# PixelConvert — Project Guidelines

## Code Style

- **Language**: Rust with GTK4/Libadwaita (`gtk4` 0.9, `libadwaita` 0.7)
- **UI**: Programmatic construction via `ObjectSubclass` pattern — no XML `.ui` templates
- **Module structure**: Flat — all modules live directly in `src/`
- **Error handling**: `anyhow::Result` with `.context()` for conversion; `BatchProgress::Failed` for per-job errors in batch processing
- **Signal connections**: `glib::clone!` with `#[weak]`/`#[strong]`/`#[upgrade_or]` to prevent reference cycles
- **Naming**: Types `PascalCase`, functions/files `snake_case`, app ID `org.pinkpixel.PixelConvert`
- **Formatting/linting**: Default `cargo fmt` and `cargo clippy` — no `rustfmt.toml` or `clippy.toml`

## Architecture

```
main.rs         → App bootstrap, keyboard shortcuts, application-level actions
window.rs       → GTK4 ObjectSubclass window (all UI state, widgets, conversion orchestration)
converter.rs    → Single-file image conversion engine (SupportedFormat enum, ImageConverter)
batch.rs        → Parallel batch processor (std::thread + mpsc channels, custom Semaphore)
preferences.rs  → Preferences window stub (planned v1.1, not wired to UI)
preview.rs      → Image preview stub (planned v1.1, not wired to UI)
```

### Threading Model — No Tokio

GTK4 uses the GLib main loop. **Never use Tokio or any async runtime.** All background work uses:

- `std::thread::scope` + `std::thread::spawn` for parallel image conversion
- `std::sync::mpsc` channels to send progress from workers to UI
- `glib::timeout_add_local` (50ms polling) to update widgets from the main thread
- Custom `Semaphore` (Mutex + Condvar) to limit concurrency to CPU core count

### Image Processing Pipeline

- `image` crate (`default-features = false`, explicit features) for most formats
- `webp::Encoder` for WebP encoding with quality parameter
- `ravif::Encoder` + `rav1e` for AVIF encoding (quality, alpha quality, speed, threads)
- `rgb` crate for pixel type conversions needed by ravif

## Build and Test

### Development

```bash
cargo build              # Debug build
cargo build --release    # Release build (LTO, strip, codegen-units=1)
cargo run                # Build and launch
cargo check              # Type-check only
cargo clippy             # Lint
cargo fmt                # Format
cargo test               # Unit tests
```

### System Dependencies

GTK4 4.12+, Libadwaita 1.5+, NASM (for rav1e/AVIF compilation), pkg-config.

```bash
# Arch Linux
sudo pacman -S rust gtk4 libadwaita meson nasm
# Debian/Ubuntu
sudo apt install libgtk-4-dev libadwaita-1-dev meson nasm
```

### Meson (System/Flatpak Integration)

```bash
meson setup builddir
meson compile -C builddir
meson install -C builddir
```

`src/meson.build` invokes `cargo build` as a custom_target. Post-install script compiles GSettings schemas, updates icon/desktop caches.

### Flatpak

```bash
flatpak-builder --user --install --force-clean build-dir org.pinkpixel.PixelConvert.yml
flatpak run org.pinkpixel.PixelConvert
```

- **Runtime**: `org.gnome.Platform` 47 + `org.freedesktop.Sdk.Extension.rust-stable`
- **Bundled native libs**: libwebp 1.4.0, libheif 1.18.2, dav1d 1.4.3, rav1e 0.7.1
- **Vendored sources**: `pixelconvert-cargo-sources.json` and `rav1e-cargo-sources.json` must be regenerated with `flatpak-cargo-generator` whenever `Cargo.lock` changes
- **Filesystem access**: `xdg-documents`, `xdg-pictures`, `xdg-download`
- **Display**: Wayland + fallback X11, DRI for GPU

### Testing

- Unit tests in `converter.rs` (`test_format_extensions`)
- Meson validates `.desktop`, AppStream metainfo XML, and GSettings schema
- Manual testing checklist in CONTRIBUTING.md (launch, drag-drop, all formats, dark mode, shortcuts)

## Conventions

### Adding a New Image Format

1. Add variant to `SupportedFormat` enum in `converter.rs`
2. Implement `extension()`, `mime_type()`, `display_name()` match arms
3. Add to `all()` vector and add save logic in `save_image()`
4. If needed, create a dedicated `save_xyz()` method
5. Update format dropdown in `window.rs` `ObjectImpl::constructed()`
6. Add MIME type and suffix to file chooser filter in `open_file_chooser()`
7. Add extension to `add_file()` validation list
8. Enable the feature flag in `Cargo.toml` `image` crate if applicable
9. Update format index mapping in `start_conversion()`

### Key Pitfalls

- **`image` crate features are explicit** — `default-features = false`. Must add feature flags for new codecs or they silently fail
- **Output overwrites originals** when converting to the same format extension (no guard implemented yet)
- **GSettings schema exists but is disconnected** — `data/org.pinkpixel.PixelConvert.gschema.xml` defines keys but `preferences.rs` has no GSettings read/write calls
- **Unused deps**: `thiserror` and `once_cell` declared in `Cargo.toml` but not imported in source
- **`#[allow(dead_code)]`** on stubs (`preferences.rs`, `preview.rs`) — these are planned v1.1 features
- **Flatpak vendor sources** must be regenerated after any `Cargo.lock` change

### Data Files

| File                                              | Purpose                                     |
| ------------------------------------------------- | ------------------------------------------- |
| `data/org.pinkpixel.PixelConvert.desktop.in`      | Desktop entry for app launchers             |
| `data/org.pinkpixel.PixelConvert.metainfo.xml.in` | AppStream metadata for software centers     |
| `data/org.pinkpixel.PixelConvert.gschema.xml`     | GSettings schema for persistent preferences |
| `data/icons/org.pinkpixel.PixelConvert.svg`       | Application icon                            |
