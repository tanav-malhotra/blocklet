# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2025-10-13

### Changed
- **BREAKING**: Solid font now renders at 7 lines tall (matching shadow version) instead of 5 lines
- Solid font now converts box drawing characters to spaces for cleaner appearance
- Both shadow and solid fonts now have consistent layout and character structure
- Improved word separation with proper spacing between letters

### Fixed
- Font height consistency between shadow and solid versions
- Character spacing for better readability in multi-word text

## [0.1.0] - 2025-10-13

### Added
- Initial release of Blocklet
- Beautiful Unicode block-based ASCII art generation
- Shadow font with proper descenders (7 lines tall)
- Solid font without shadows (5 lines tall)
- Multi-line support via multiple arguments
- Proper typographic descenders for Q, comma, question mark
- Word wrapping with configurable width
- Cross-platform support (Windows, macOS, Linux)
- Three font variants: standard_shadow, standard_solid, standard
- Command-line options for font selection and shadow toggling
- Full alphabet (A-Z), numbers (0-9), and basic punctuation support
- Zero runtime dependencies - single binary
- Comprehensive test suite and benchmarks

### Features
- 🎨 Unicode block characters with box-drawing
- 🌟 Drop-shadow effects
- 📝 Multi-line text rendering
- ⬇️ Proper descenders
- 📏 Automatic word wrapping
- ⚡ Fast performance
- 🔀 Cross-platform compatibility

[0.1.1]: https://github.com/tanav-malhotra/blocklet/releases/tag/v0.1.1
[0.1.0]: https://github.com/tanav-malhotra/blocklet/releases/tag/v0.1.0

