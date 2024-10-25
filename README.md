<div align="center">

<img src="assets/logo.svg" width=400>

## `sweeten` your daily `iced` brew

[![Made with iced](https://iced.rs/badge.svg)](https://github.com/iced-rs/iced)

</div>

## Overview

`sweeten` provides enhanced versions of common `iced` widgets with additional functionality for more complex use cases. It aims to maintain the simplicity and elegance of `iced` while offering "sweetened" variants with extended capabilities.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
sweeten = "0.1.0"
```

## Current Features

### `MouseArea`

An enhanced version of `iced`'s mouse interaction handling with additional capabilities:

- `on_press_with`: Capture click position with a closure

## Examples

For a complete example, see [`examples/mouse_area.rs`](examples/mouse_area.rs).

You can run it with:

```bash
cargo run --example mouse_area
```

## Code Structure

The library is organized into modules for each enhanced widget:

- `widget/`: Contains all widget implementations
  - `mouse_area.rs`: Enhanced mouse interaction handling
  - (more widgets coming soon!)

## Planned Features

- [x] MouseArea widget
- [ ] Row and Column with drag and drop and enhanced layout capabilities

## Contributing

Contributions are welcome! If you have ideas for new widgets or enhancements:

1. Fork the repository
2. Create a feature branch
3. Implement your changes with tests
4. Submit a PR

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgements

- [iced](https://github.com/iced-rs/iced)
- [Rust programming language](https://www.rust-lang.org/)