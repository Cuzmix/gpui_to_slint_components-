# Spec: MultiSelect

Source: `ely-core/src/forms/multi.rs`.

- A wrapping row, items centered, 4 px gap, at least control height, 4 px before, control padding after, 2 px above and below; radius md, 1 px border (focus while focused, else border-strong), surface fill, base text, pointer cursor. Panics with no choices. A tab stop.
- Chips: a removable Tag per chosen choice, in choice order; removing one turns it off. With none chosen, the placeholder ("Choose…") 8 px in, fg-subtle. A chevrons-up-down icon (xs, fg-subtle) pushed to the end of its line.
- A mouse click toggles the list, its cursor at the first enabled chosen row, else the first enabled row. Keys: Backspace with values drops the last value chosen (in chosen order). Closed: Down, Up, Enter or Space open. Open: Up/Down/Home/End walk past disabled rows, Enter or Space toggle the row (the list stays open, the cursor on it), Escape closes. Blur closes. A floating list with a check column checking every chosen value; a press outside closes it. Each change reports the values (logs).
- Role combo box named by the label (or placeholder), valued by the chosen labels joined by ", ".

## Slint mapping
- `MultiSelect { owner, choices, selected <=>, placeholder, label, size; selection-changed(selected) }`, out `has-focus`, `open`, `highlighted`. `selected` is in-out (G48).
- Chips, toggles and the value text run in Rust (`Choices.among`, `toggled`, `without`, `joined`, `wire_options!`; `Toggles.contains`, `wire_toggles!`). Keys share `ListKeys.moved` with Select. The list goes through `OptionAnchor`/`OptionLayer` (G54). No choices never opens (gpui panics).
