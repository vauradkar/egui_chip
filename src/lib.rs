//! # Egui Chip
//!
//! `egui_chip` is a library for creating and configuring `ChipEdit` instances
//! in the `egui` framework. The crate is inspired from Material's [chip](https://m3.material.io/components/chips/overview).
//! This crate provides a fluent interface for setting
//! various options and parameters for `ChipEdit` components.
//!
//! ## Usage
//!
//! Add this crate to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! egui_chip = "0.1.0"
//! ```
//!
//! ## Examples
//!
//! ```rust
//! use egui_chip::ChipEditBuilder;
//!
//! let chip_edit = ChipEditBuilder::new(", ")
//!     .unwrap()
//!     .frame(true)
//!     .texts(["hello", "world"])
//!     .build();
//! ```
//!
//! ## Features
//!
//! - Customizable options for appearance
//! - Supports moving from one chip to another
//! - Supports deleting chip with delete or backspace keys
//! - Integration with the `egui` framework
#![warn(clippy::all)]

mod builder;
mod chip;
mod chip_edit;
mod output;
mod state;
mod unowned_chip_edit;

pub use builder::ChipEditBuilder;
pub use chip_edit::ChipEdit;
pub use output::ChipEditOutput;
pub use unowned_chip_edit::UnownedChipEdit;
