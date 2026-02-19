# PixelConvert

<div align="center">
  <img src="logo.png" width="300" height="300" alt="PixelConvert Logo">
  
  **A modern, fast, and beautiful image conversion tool for Linux**
  
  [![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
  [![Built with GTK4](https://img.shields.io/badge/Built%20with-GTK4-blue)](https://gtk.org)
  [![Rust](https://img.shields.io/badge/Rust-1.75+-orange)](https://www.rust-lang.org)
</div>

## Features

✨ **Modern Image Formats**

- Support for PNG, JPEG, WebP, AVIF, GIF, BMP, TIFF, and ICO formats
- High-quality encoding with customizable quality settings
- Optimized for modern formats like WebP and AVIF

⚡ **Fast Batch Processing**

- Convert multiple images simultaneously
- Parallel processing with configurable thread pool
- Real-time progress tracking

🎨 **Beautiful Interface**

- Clean and intuitive GTK4/Libadwaita UI
- Automatic light/dark mode following system theme
- Drag-and-drop support for easy file selection

🔧 **Flexible Controls**

- Adjustable quality slider (0-100)
- Choose from 8 popular image formats
- Batch file management

## Screenshots

_Coming soon_

## Installation

### Flatpak (Recommended)

PixelConvert will be available on Flathub soon:

```bash
flatpak install flathub org.pinkpixel.PixelConvert
```

### Build from Source

#### Requirements

- Rust 1.75 or later
- GTK4 (4.12+)
- Libadwaita (1.5+)
- Meson build system
- NASM (for AVIF/rav1e compilation)

```bash
# Install build dependencies (Arch Linux)
sudo pacman -S rust gtk4 libadwaita meson nasm

# Clone the repository
git clone https://github.com/pinkpixel-dev/pixelconvert.git
cd pixelconvert

# Build with Cargo
cargo build --release

# Or build Flatpak locally
flatpak-builder --user --install --force-clean build-dir org.pinkpixel.PixelConvert.yml
```

## Usage

### Basic Workflow

1. **Open Files**: Click "Select Files" or drag-and-drop images into the window
2. **Choose Format**: Select your desired output format from the dropdown
3. **Adjust Quality**: Use the quality slider to balance size vs quality
4. **Convert**: Click "Convert Images" to process your files

### Keyboard Shortcuts

| Action             | Shortcut            |
| ------------------ | ------------------- |
| Open Files         | `Ctrl+O`            |
| Convert Images     | `Ctrl+Enter`        |
| Clear Files        | `Ctrl+Shift+Delete` |
| Preferences        | `Ctrl+,`            |
| Keyboard Shortcuts | `Ctrl+?`            |
| Quit               | `Ctrl+Q`            |

## Supported Formats

| Format | Read | Write | Notes                             |
| ------ | :--: | :---: | --------------------------------- |
| PNG    |  ✅  |  ✅   | Lossless compression              |
| JPEG   |  ✅  |  ✅   | Lossy compression                 |
| WebP   |  ✅  |  ✅   | Modern format, smaller sizes      |
| AVIF   |  ✅  |  ✅   | Next-gen format, best compression |
| GIF    |  ✅  |  ✅   | Animation not yet supported       |
| BMP    |  ✅  |  ✅   | Uncompressed                      |
| TIFF   |  ✅  |  ✅   | Professional format               |
| ICO    |  ✅  |  ✅   | Windows icons                     |

## Technology Stack

- **Language**: Rust 🦀
- **UI Framework**: GTK4 + Libadwaita
- **Image Processing**:
  - `image` crate for core formats
  - `webp` for WebP encoding
  - `ravif` + `rav1e` for AVIF encoding
  - `rgb` for color space conversion
- **Async Runtime**: `std::thread` + `mpsc` channels (no Tokio — GTK4 uses the GLib main loop)
- **Build System**: Meson + Cargo
- **Distribution**: Flatpak

## Roadmap

### Version 1.0 (Current)

- [x] Core image conversion engine
- [x] Batch processing system
- [x] GTK4/Libadwaita UI
- [x] Drag-and-drop support
- [x] Progress tracking
- [x] Keyboard shortcuts
- [x] Custom output directory selection
- [x] Documentation
- [ ] Flathub release

### Version 1.1 (Planned)

- [ ] Image preview with before/after comparison
- [ ] Metadata preservation options
- [ ] Image resizing capabilities
- [ ] File naming patterns
- [ ] Preferences persistence via GSettings
- [ ] Multi-language support

### Version 1.2+ (Future)

- [ ] Advanced compression options per format
- [ ] Animated GIF/WebP support
- [ ] Bulk rename operations
- [ ] Image optimization presets
- [ ] CLI support for scripting

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## Credits

- Built with love by the PinkPixel Team
- Icons and design inspired by GNOME design guidelines
- Uses the amazing Rust ecosystem and GTK tooling

## Support

- **Issues**: [GitHub Issues](https://github.com/pinkpixel-dev/pixelconvert/issues)
- **Discussions**: [GitHub Discussions](https://github.com/pinkpixel-dev/pixelconvert/discussions)
- **Website**: https://pinkpixel.org

---

<div align="center">
  Made with ❤️ using Rust and GTK4
</div>
