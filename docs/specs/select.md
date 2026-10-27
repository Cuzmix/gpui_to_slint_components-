# Spec: Select (and the field button)

Source: `ely-core/src/forms/select.rs` (`field_button`, `field_text`, `listing`, `Select`).

- Field button: control height, control padding, 8 px gap, radius md, 1 px border (focus while focused, else border-strong), surface fill (sunken when disabled), the size's text size, pointer cursor unless disabled. A tab stop unless disabled; a press focuses it.
- Field text: the value, or the placeholder in fg-subtle (fg-disabled when disabled), taking the rest, cut with an ellipsis.
- Select: the chosen row's icon (sm, fg-muted) or avatar (xs), its label or the placeholder ("Choose…"), and a chevrons-up-down icon (xs, fg-subtle). Panics with no choices unless disabled. Role combo box named by `label` (or the placeholder), valued by the chosen label, expanded while open; "Unavailable" when disabled.
- `listing`: a press toggles the list, its cursor at the choice or the first enabled row. Closed: Down, Up, Enter or Space open. Open: Down/Up step past disabled rows (wrapping), Home/End go to the first/last enabled, Enter or Space pick, Escape closes. Leaving the field closes. A floating list (`Popup`) with a check column checking the choice; a press outside closes it. A pick closes it and reports the value (logs).

## Slint mapping
- `FieldButton { size, enabled; pressed, key-pressed }` with the row as `@children`; `FieldText { text, placeholder, size, enabled }`; `ListKeys.moved` (pure Slint) holds the list keys for every opener.
- `Select { owner, choices, selected <=>, placeholder, label, size, enabled, unavailable-text; selection-changed(value) }`, out `has-focus`, `open`, `highlighted`. `selected` is in-out (G48). The list goes through `OptionAnchor`/`OptionLayer` (G54) and needs `wire_options!` and `wire_toggles!`.
- No choices shows the placeholder and never opens (gpui panics). The avatar waits for Avatar (chapter 12).
