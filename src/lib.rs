//! Rust support for the Ely Slint components.
//!
//! The `.slint` library under `ui/` works without this crate. Components that need logic Slint
//! cannot express (key syntax parsing, date math, large models) take plain values or models,
//! and this crate computes them. Nothing here holds UI state.

pub mod columns;
#[cfg(feature = "desktop")]
pub mod desktop;
pub mod dock;
pub mod fields;
pub mod focus;
pub mod format;
pub mod keys;
pub mod layout;
pub mod numbers;
pub mod open;
pub mod options;
pub mod panes;
pub mod paths;
pub mod text;
pub mod toggles;
pub mod window;
pub mod windows;
pub mod workspace;
#[doc(hidden)]
pub mod wire;

#[doc(hidden)]
pub use log;

pub use keys::{Keystroke, KeystrokeError, Platform, key_label, keystroke, keystroke_labels, shortcut_text};
