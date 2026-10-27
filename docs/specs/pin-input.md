# Spec: PinInput

Source: `ely-core/src/forms/pin.rs`.

- `length` boxes (panics on zero), digits only unless `letters`; the code reads upper case. A hidden text input (max `length` characters, filtered) lies over the boxes at zero opacity and takes typing, paste and the pointer. Its caret is kept at the end unless all is selected.
- Each box: lg control × 1.25 square, radius md, 1 px border: focus on the active box (the next to fill, or the last) while focused, border-strong otherwise; selection fill on filled boxes while all is selected, else surface. Glyph xl medium fg; `masked` shows • in filled boxes. An empty active box shows the caret (caret width × xl text size, focus color), blinking every 530 ms while focused (steady under reduced motion; an edit shows it again).
- Boxes 8 px apart. Every box filled: `on_complete(code)` (logs). A new `attempt` clears the boxes (logs).

## Slint mapping
- `PinInput { length, label, letters, masked, attempt, enabled; completed(code) }`, out `code`. The field is a TextField with kind `digits` or `alnum` (registered by `wire_fields!`); its characters come from `TextFields.chars`. Zero boxes shows one (gpui panics).
