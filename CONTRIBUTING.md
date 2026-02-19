# Contributing to PixelConvert

Thank you for your interest in contributing to PixelConvert! This document provides guidelines and instructions for contributing.

## Code of Conduct

This project adheres to a code of conduct that all contributors are expected to follow:

- Be respectful and inclusive
- Welcome diverse perspectives
- Focus on constructive feedback
- Help create a positive community

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check existing issues to avoid duplicates. When you create a bug report, include as many details as possible:

- **Use a clear and descriptive title**
- **Describe the exact steps to reproduce the problem**
- **Provide specific examples**
- **Describe the behavior you observed and what you expected**
- **Include screenshots if relevant**
- **Note your environment**: OS, GTK version, Flatpak version, etc.

### Suggesting Enhancements

Enhancement suggestions are welcome! When suggesting an enhancement:

- **Use a clear and descriptive title**
- **Provide a detailed description of the proposed feature**
- **Explain why this enhancement would be useful**
- **Include mockups or examples if applicable**

### Pull Requests

1. Fork the repository
2. Create a new branch from `main`
3. Make your changes
4. Add or update tests as needed
5. Ensure all tests pass
6. Update documentation if needed
7. Submit a pull request

## Development Setup

### Prerequisites

```bash
# Arch Linux
sudo pacman -S rust gtk4 libadwaita meson nasm git

# Ubuntu/Debian
sudo apt install rustc cargo libgtk-4-dev libadwaita-1-dev meson nasm git

# Fedora
sudo dnf install rust cargo gtk4-devel libadwaita-devel meson nasm git
```

### Building

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/pixelconvert.git
cd pixelconvert

# Build and run
cargo build
cargo run

# Run tests
cargo test

# Check code
cargo check
cargo clippy
```

### Flatpak Development

```bash
# Install flatpak-builder
sudo pacman -S flatpak-builder

# Build Flatpak
flatpak-builder --user --install --force-clean build-dir dev.pinkpixel.PixelConvert.yml

# Run Flatpak
flatpak run dev.pinkpixel.PixelConvert
```

## Coding Guidelines

### Rust Style

- Follow standard Rust formatting: `cargo fmt`
- Run clippy and fix warnings: `cargo clippy`
- Write meaningful commit messages
- Add comments for complex logic
- Write unit tests for new functionality

### GTK/UI Guidelines

- Follow GNOME Human Interface Guidelines (HIG)
- Ensure dark mode compatibility
- Test with both light and dark themes
- Use Libadwaita widgets when possible
- Keep UI responsive (use async for long operations)

### Commit Messages

- Use the present tense ("Add feature" not "Added feature")
- Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
- Limit the first line to 72 characters
- Reference issues and pull requests when relevant

Example:

```
Add JPEG XL format support

- Implement encoder using libjxl
- Add format to dropdown menu
- Update tests and documentation

Closes #42
```

## Project Structure

```
pixelconvert/
├── src/
│   ├── main.rs           # Application entry point
│   ├── window.rs         # Main window UI
│   ├── converter.rs      # Image conversion engine
│   ├── batch.rs          # Batch processing system
│   ├── preferences.rs    # Preferences window
│   └── preview.rs        # Preview widget
├── data/
│   ├── icons/            # Application icons
│   ├── *.desktop.in      # Desktop entry
│   ├── *.metainfo.xml.in # AppStream metadata
│   └── *.gschema.xml     # GSettings schema
├── build-aux/
│   └── meson/            # Build scripts
├── dev.pinkpixel.PixelConvert.yml  # Flatpak manifest
└── meson.build           # Build configuration
```

## Testing

### Manual Testing Checklist

Before submitting a PR, please test:

- [ ] Application launches without errors
- [ ] File selection dialog works
- [ ] Drag-and-drop works
- [ ] Format selection works for all formats
- [ ] Quality slider updates correctly
- [ ] Batch conversion completes successfully
- [ ] Progress bar updates during conversion
- [ ] Error handling for invalid files
- [ ] Dark mode works correctly
- [ ] Keyboard shortcuts work
- [ ] About dialog displays correctly

### Automated Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

## Documentation

- Update README.md for new features
- Add entries to CHANGELOG.md
- Document public APIs with rustdoc comments
- Update Flatpak manifest if dependencies change

## Need Help?

- Open an issue with your question
- Check existing issues and discussions
- Join community discussions

## Recognition

Contributors will be recognized in:

- CHANGELOG.md
- About dialog (future releases)
- GitHub contributors page

Thank you for contributing to PixelConvert! 🎉
