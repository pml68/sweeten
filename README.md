<div align="center">

<img src="assets/logo.svg" width=400>

## `sweeten` your daily `iced` brew

[![Made with iced](https://iced.rs/badge.svg)](https://github.com/iced-rs/iced)

</div>

## Overview

`sweeten` provides sweetened versions of common `iced` widgets with additional
functionality for more complex use cases. It aims to maintain the simplicity and
elegance of `iced` while offering "sweetened" variants with extended
capabilities.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
sweeten = "0.1.0"
```

## Current Features

### `MouseArea`

A sweetened version of `iced`'s `mouse_area` widget with an additional
`on_press_with` method for capturing the click position with a closure. Use it
like:

```rust
mouse_area("Click me and I'll tell you where!",)
    .on_press_with(|point| Message::ClickWithPoint(point)),
```

### `PickList`

A sweetened version of `iced`'s `PickList` which accepts an optional closure to
disable some items. Use it like:

```rust
pick_list(
    &Language::ALL[..],
    Some(|languages: &[Language]| {
        languages
            .iter()
            .map(|lang| matches!(lang, Language::Javascript))
            .collect()
    }),
    self.selected_language,
    Message::Pick,
)
.placeholder("Choose a language...");
```

> Note that the compiler is not currently able to infer the type of the closure,
> so you may need to specify it explicitly as shown above.

## Examples

For complete examples, see [`examples/`](examples/) or run an example like this:

```bash
cargo run --example mouse_area
```

## Code Structure

The library is organized into modules for each enhanced widget:

- `widget/`: Contains all widget implementations
  - `mouse_area.rs`: Sweetened mouse interaction handling
  - `pick_list.rs`: Sweetened pick list with item disabling
  - (more widgets coming soon!)

## Planned Features

- [x] MouseArea widget
- [x] PickList widget
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