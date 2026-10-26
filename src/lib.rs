//! Rust support for the Ely Slint components.
//!
//! The `.slint` library under `ui/` works without this crate. Components that need logic Slint
//! cannot express (key syntax parsing, date math, large models) take plain values or models,
//! and this crate computes them. Nothing here holds UI state.

pub mod keys;

pub use keys::{Keystroke, KeystrokeError, Platform, key_label, keystroke, keystroke_labels, shortcut_text};
