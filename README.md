# egui_chip: compact component to display tags, selections, or actions

[![docs.rs](https://img.shields.io/docsrs/egui_chip)](https://docs.rs/egui_chip)
[![CI Status](https://img.shields.io/github/actions/workflow/status/vauradkar/egui_chip/rust.yml)](https://github.com/vauradkar/egui_chip/actions?workflow=CI)
[![Crates.io Version](https://img.shields.io/crates/v/egui_chip)](https://crates.io/crates/egui_chip)
[![Crates.io License](https://img.shields.io/crates/l/egui_chip)](<https://github.com/vauradkar/egui_chip/blob/main/LICENSE-MIT>)
[![Crates.io License](https://img.shields.io/crates/l/egui_chip)](<https://github.com/vauradkar/egui_chip/blob/main/LICENSE-APACHE>)

## Features

- Customizable options for appearance
- Supports moving from one chip to another
- Supports deleting chip with delete or backspace keys
- Integration with the `egui` framework

## Example

```rust
use egui_chip::ChipEditBuilder;

let chip_edit = ChipEditBuilder::new(", ")
    .unwrap()
    .frame(true)
    .texts(["hello", "world"])
    .build();
```

## Sample app

```shell
cargo run --example simple
```

<img src="demo/demo_0.1.0.gif" width="80%">
