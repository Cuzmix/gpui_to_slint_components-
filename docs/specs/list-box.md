# Spec: ListBox

Source: `ely-core/src/forms/listbox.rs`.

- A tab stop, scrolling column filling its parent's height up to list-max-height; 4 px padding, radius lg, 1 px border (focus while focused, else border), surface fill, sm text. Panics with no choices.
- Rows: option rows with a check column, checked for chosen values; the cursor row is highlighted only while the list has focus.
- A click on an enabled row chooses it and moves the cursor there. Down/Up step past disabled rows (wrapping), Home/End go to the first/last enabled row, the cursor kept in view; Enter or Space on an enabled cursor row choose it. One row replaces the choice; with `multiple` a row toggles, in choice order. Each choice reports the values (logs). Role list box, named by `label`.

## Slint mapping
- `ListBox { choices, selected <=>, multiple, label; selection-changed(selected) }`, out `has-focus`, `highlighted`. `selected` is in-out (G48). Edits run in Rust (`Choices.toggled`, `Choices.step`, `wire_options!`; `Toggles.contains`, `wire_toggles!`).
- Rows are `OptionRow`s with `pick-on-press: false`, so they choose on the click and a press focuses the list. The role is list (Slint 1.18 has no list box role).
