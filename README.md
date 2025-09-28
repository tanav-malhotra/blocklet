# Blocklet

A cross-platform CLI tool that generates ASCII art using Unicode block characters, similar to figlet but with beautiful solid Unicode blocks instead of outlines or hash symbols.

[![Rust](https://github.com/tanav-malhotra/blocklet/workflows/Rust/badge.svg)](https://github.com/tanav-malhotra/blocklet/actions)
[![License: GPL-3.0](https://img.shields.io/badge/License-GPL--3.0-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

## ✨ Features

- 🎨 **Unicode Block Characters**: Uses solid Unicode blocks (█) instead of outlines or hash symbols
- 🌟 **Drop-Shadow Effect**: Beautiful drop-shadows enabled by default (disable with `-n`)
- 📏 **Text Wrapping**: Automatic word wrapping with configurable width limits
- 🔀 **Cross-Platform**: Works on Windows, macOS, and Linux
- ⚡ **Fast Performance**: Optimized rendering engine with comprehensive benchmarks
- 🧪 **Well Tested**: Extensive unit tests ensuring reliability
- 📦 **Zero Runtime Dependencies**: Single binary with everything built-in

## 🚀 Quick Start

### Installation

Install from crates.io:
```bash
cargo install blocklet
```

Or build from source:
```bash
git clone https://github.com/tanav-malhotra/blocklet.git
cd blocklet
cargo install --path .
```

### Basic Usage

```bash
# Simple text rendering (with drop-shadow by default)
blocklet "Hello World"

# Disable drop-shadow
blocklet "Clean Text" --no-shadow

# Short form to disable shadow
blocklet "Clean Text" -n

# With width limiting (word wrapping)
blocklet "This is a long text that will wrap" --width 40
```

## 🔧 Command Line Options

```
blocklet [OPTIONS] <TEXT>

ARGUMENTS:
    <TEXT>    The text to convert to ASCII art

OPTIONS:
    -w, --width <WIDTH>      Maximum width for output (0 = no limit) [default: 0]
    -H, --height <HEIGHT>    Font height in characters [default: 6]
    -n, --no-shadow          Disable drop-shadow effect
    -h, --help               Print help information
    -V, --version            Print version information
```

## 🎨 Font

Blocklet uses a single, well-tuned `standard` Unicode block font optimized for readability.

## 🏗️ Architecture

### Core Components

1. **Font System** (`src/font.rs`)
   - Font character definitions using Unicode blocks
   - Single, readable Unicode block font (standard)
   - Character fallback system

2. **Rendering Engine** (`src/renderer.rs`)
   - Text-to-blocks conversion
   - Word wrapping functionality
   - Multi-line text support

3. **CLI Interface** (`src/main.rs`)
   - Command-line argument parsing
   - Font selection and options
   - Error handling

## 🧪 Testing

Run the test suite:
```bash
# Unit tests
cargo test

# Performance benchmarks
cargo bench

# Check test coverage
cargo tarpaulin --out Html
```

### Test Categories

- **Unit Tests**: Individual component testing
- **Integration Tests**: End-to-end functionality
- **Performance Benchmarks**: Rendering speed optimization
- **Cross-platform Tests**: Windows, macOS, Linux compatibility

## 📊 Performance

Benchmarks on modern hardware show excellent performance:

- **Short text (2 chars)**: ~51ns
- **Medium text (11 chars)**: ~79ns
- **Long text (43 chars)**: ~80ns
- **Very long text (144 chars)**: ~107ns

Run your own benchmarks with:
```bash
cargo bench --bench rendering
```

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Setup

1. **Prerequisites**:
   - Rust 1.70+ (2021 edition)
   - Git

2. **Clone and build**:
   ```bash
   git clone https://github.com/tanav-malhotra/blocklet.git
   cd blocklet
   cargo install --path .
   ```

3. **Run tests**:
   ```bash
   cargo test
   cargo bench
   ```

4. **Code style**:
   ```bash
   cargo fmt
   cargo clippy
   ```

### Adding New Fonts

1. Create character definitions in `src/font.rs`
2. Add the font to the `FONTS` HashMap
3. Add tests for the new font
4. Update documentation

## 📜 License

This project is licensed under the GNU General Public License v3.0.

See [LICENSE](LICENSE) or [https://www.gnu.org/licenses/gpl-3.0.html](https://www.gnu.org/licenses/gpl-3.0.html) for details.

at your option.

## 🙋 FAQ

### Q: How is this different from figlet?

**A**: While figlet uses ASCII characters to create outlined text, Blocklet uses solid Unicode block characters (█) for a more modern, filled appearance. It's designed to look better in modern terminals with Unicode support.

### Q: Does it work on all terminals?

**A**: Yes, Blocklet works on any terminal that supports Unicode (UTF-8). This includes most modern terminals on Windows, macOS, and Linux.

### Q: Can I create custom fonts?

**A**: Currently, fonts are built into the binary. Custom font support is planned for a future release.

### Q: What about colored output?

**A**: Colored output is planned for a future release. Currently, the tool outputs monochrome Unicode blocks.

### Q: How do I report bugs or request features?

**A**: Please open an issue on our [GitHub repository](https://github.com/tanav-malhotra/blocklet/issues).

## 🔗 Related Projects

- [figlet](http://www.figlet.org/) - Original ASCII art text generator
- [toilet](http://caca.zoy.org/wiki/toilet) - Another ASCII art text generator
- [banner](https://www.unix.com/man-page/linux/1/banner/) - Classic UNIX text banner utility

## 📝 Changelog

See [CHANGELOG.md](CHANGELOG.md) for a detailed history of changes.

---

**Made with ❤️ in Rust** 🦀
