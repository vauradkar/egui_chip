# egui_chip: compact component to display tags, selections, or actions

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
