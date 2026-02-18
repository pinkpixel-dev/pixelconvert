# Plan: PixelConvert - Modern Image Converter for Linux

Build a high-performance batch image converter in Rust with GTK4/Libadwaita that supports all modern formats (WebP, AVIF, HEIF, and traditional formats). Features quality controls, image preview, and automatic system theme support. Publish to Flathub for Linux users.

**Tech Stack:** Rust + GTK4 + Libadwaita + image crate ecosystem

---

## Steps

### Phase 1: Project Foundation (parallel with Phase 2)

1. **Initialize Rust project structure**
   - Create Cargo workspace with proper dependencies (gtk4 v0.9+, libadwaita v0.7+, image, webp, ravif crates)
   - Set up meson build system for Flatpak integration
   - Configure project as Cargo + meson hybrid (standard for Flatpak Rust apps)
2. **Create Flatpak manifest** (_parallel with step 1_)
   - Create `org.pinkpixel.PixelConvert.yml` with GNOME 47 runtime
   - Define modules for image format libraries (libwebp, libavif/rav1e, libheif)
   - Configure finish-args for filesystem access (xdg-documents, xdg-pictures, xdg-download)
   - Set up GPU acceleration (--device=dri) for better performance

3. **Create application metadata files** (_parallel with steps 1-2_)
   - `org.pinkpixel.PixelConvert.desktop` - Desktop entry with MIME types for supported formats
   - `org.pinkpixel.PixelConvert.metainfo.xml` - AppStream metadata with app description, screenshots, release info
   - `org.pinkpixel.PixelConvert.gschema.xml` - GSettings schema for preferences (dark mode, default quality, etc.)
   - Design app icon following GNOME HIG (SVG + exported PNGs at standard sizes)

### Phase 2: Core Image Processing (_depends on step 1_)

4. **Implement format conversion engine**
   - Create `converter.rs` module using Rust `image` crate as base
   - Integrate `webp` crate for WebP encoding/decoding with quality control
   - Integrate `ravif` crate for AVIF encoding (uses rav1e encoder)
   - Integrate `libheif-sys` bindings for HEIF/HEIC support
   - Implement quality/compression settings per format (WebP: quality + method, AVIF: quality + speed)
   - Add error handling for unsupported formats and corrupted files
5. **Implement batch processing system** (_depends on step 4_)
   - Create async batch processor using `tokio` or `async-std`
   - Process images in parallel with configurable thread pool
   - Implement progress tracking (processed count, current file, errors)
   - Add cancellation support for long-running batches

### Phase 3: UI Implementation (_depends on Phase 1 complete_)

6. **Create main application window**
   - Use GTK4-rs + Libadwaita-rs bindings
   - Implement `AdwApplicationWindow` with automatic system theme support
   - Create empty state using `AdwStatusPage` with drag-and-drop instructions
   - Add drag-and-drop controller for file input (GTK4 `DropTarget`)
   - Implement file picker dialog with MIME type filters for supported formats

7. **Build conversion controls UI** (_parallel with step 6_)
   - Create format selection dropdown (ComboBox) with all supported formats
   - Add quality slider (GtkScale, 0-100 range) with live value display
   - Create format-specific options (WebP method, AVIF speed, JPEG progressive, etc.)
   - Add preset buttons (High Quality, Balanced, Small Size)
   - Include output location selector with default behavior
8. **Implement preview functionality** (_depends on step 6_)
   - Create split-view preview area with before/after comparison
   - Show original image info (dimensions, format, file size)
   - Show estimated output info (new format, estimated size)
   - Add zoom/pan controls for large images
   - Implement thumbnail generation for batch preview

9. **Create batch processing UI** (_depends on steps 5, 6_)
   - Build file list view showing selected images (GtkListView or GtkColumnView)
   - Add per-file format override option
   - Show progress bar during batch conversion (GtkProgressBar)
   - Display real-time processing status (spinner + current file name)
   - Show completion summary (successful conversions, errors, time taken)

### Phase 4: Settings & Polish (_depends on Phase 3 complete_)

10. **Implement preferences window**
    - Create `AdwPreferencesWindow` following GNOME HIG
    - Add dark mode toggle (syncs with GSettings and system preference)
    - Add default quality/compression settings per format
    - Add default output naming patterns
    - Add option for metadata preservation (on by default)
    - Add performance settings (thread pool size)

11. **Add keyboard shortcuts and menu**
    - Implement app menu with About dialog (using `AdwAboutWindow`)
    - Add keyboard shortcuts (Ctrl+O: open, Ctrl+S: save/convert, Ctrl+,: preferences)
    - Create shortcuts window showing all keybindings (GtkShortcutsWindow)

### Phase 5: Testing & Flatpak Preparation (_depends on Phase 4 complete_)

12. **Local testing and debugging**
    - Build locally with cargo: `cargo build --release`
    - Test with `flatpak-builder`: `flatpak-builder build-dir org.pinkpixel.PixelConvert.yml --force-clean`
    - Install locally: `flatpak-builder --user --install ...`
    - Run validation: `flatpak-builder-lint manifest` and `flatpak-builder-lint appstream`
    - Test all format conversions (round-trip testing)
    - Test batch processing with large file sets
    - Test error conditions (invalid files, insufficient permissions)

13. **Documentation and assets**
    - Write comprehensive README.md with features, installation, usage, screenshots
    - Create CHANGELOG.md (initial v1.0.0 release)
    - Add CONTRIBUTING.md for community contributions
    - Create Apache 2.0 LICENSE file
    - Take screenshots for metainfo.xml (at least 3: main window, batch processing, preferences)
    - Create promotional graphics for Flathub listing

14. **Flathub submission** (_depends on steps 12-13_)
    - Fork `flathub/flathub` repository and use `new-pr` branch
    - Copy manifest files to repository
    - Ensure all linter checks pass
    - Open pull request against `new-pr` branch
    - Respond to reviewer feedback (may take several iterations)

---

## Relevant Files

**To be created:**

- `Cargo.toml` - Rust dependencies: gtk4 v0.9+, libadwaita v0.7+, image, webp, ravif, tokio
- `meson.build` - Build system integration for Flatpak
- `src/main.rs` - Application entry point, GTK application setup
- `src/window.rs` - Main window implementation using gtk4-rs and libadwaita-rs
- `src/converter.rs` - Image conversion engine with multi-format support
- `src/batch.rs` - Async batch processing system
- `src/preview.rs` - Image preview widget
- `src/preferences.rs` - Settings window
- `data/ui/window.ui` - GTK UI definition (optional, can use programmatic approach)
- `data/org.pinkpixel.PixelConvert.desktop` - Desktop entry
- `data/org.pinkpixel.PixelConvert.metainfo.xml` - AppStream metadata
- `data/org.pinkpixel.PixelConvert.gschema.xml` - Settings schema
- `data/icons/` - Application icons (SVG + PNGs)
- `org.pinkpixel.PixelConvert.yml` - Flatpak manifest
- `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `LICENSE` - Documentation

---

## Verification

1. **Format conversion accuracy**
   - Convert PNG → WebP → PNG and verify no visual degradation
   - Test all format pairs (each format to every other format)
   - Verify quality settings actually affect output file size/quality
   - Test edge cases: very large images (>100MP), animated WebP, transparency

2. **Batch processing reliability**
   - Process 100+ mixed-format files and verify all succeed
   - Test cancellation mid-batch
   - Verify error handling doesn't crash on corrupt files
   - Check memory usage doesn't grow unbounded

3. **UI/UX validation**
   - Verify system theme detection works (test in GNOME, KDE Plasma, others)
   - Test drag-and-drop from Files, Firefox, Chrome
   - Verify all keyboard shortcuts work
   - Test with screen reader for accessibility
   - Verify responsive layout at different window sizes

4. **Flatpak validation**
   - Run `flatpak-builder-lint manifest org.pinkpixel.PixelConvert.yml`
   - Run `flatpak-builder-lint appstream data/org.pinkpixel.PixelConvert.metainfo.xml`
   - Verify app runs in sandboxed Flatpak environment
   - Test file access permissions work correctly
   - Verify no runtime errors in sandboxed environment

5. **Performance testing**
   - Benchmark batch conversion of 100 images
   - Profile memory usage during large batch
   - Verify multi-threading actually parallelizes work
   - Compare performance vs ImageMagick or similar tools

---

## Decisions

**Technology Choices:**

- **Rust + GTK4**: Chosen for performance benefits in batch processing large images
- **All modern formats from start**: AVIF, HEIF, WebP included in v1.0 for comprehensive format support
- **Libadwaita**: Automatic system theme support, modern GNOME design patterns
- **Async batch processing**: Using tokio for non-blocking UI during conversion
- **Meson + Cargo hybrid build**: Standard approach for Rust Flatpak apps

**Feature Scope:**

- **Included**: Basic format conversion, quality/compression settings, batch conversion, image preview
- **Excluded for v1.0** (can add later):
  - Resize images during conversion
  - Metadata preservation (EXIF, XMP) - can add in v1.1
  - Advanced editing features (crop, rotate, filters)
  - Animation support (animated WebP/AVIF)
  - Command-line interface

**Design Decisions:**

- Follow GNOME Human Interface Guidelines for consistent UX
- Use native file dialogs and system integration
- Default output location: same directory as source file
- Format-specific options collapsed by default (advanced users can expand)
- Branding: Pink Pixel colors and "Made with ❤️ by Pink Pixel" in About dialog

**Flatpak Specifics:**

- App ID: `org.pinkpixel.PixelConvert`
- Runtime: `org.gnome.Platform` version 47
- Permissions: filesystem access to common directories, GPU for acceleration
- Icon follows GNOME icon spec with symbolic variant

---

## Further Considerations

1. **Metadata preservation** - Not in must-have features, but users may expect it. Recommend adding in v1.1 using `kamadak-exif` crate for EXIF and `quick-xml` for XMP.

2. **JPEG XL support** - Can be added in a future version (v1.2+) once Rust bindings mature. The libjxl integration complexity is deferred to reduce initial development scope.

3. **CLI interface** - Many power users prefer CLI for scripting. Recommend adding `clap`-based CLI in v1.2 that shares the same conversion engine.

4. **CI/CD** - Set up GitHub Actions to build and test Flatpak on every commit, catch issues early before Flathub submission.
