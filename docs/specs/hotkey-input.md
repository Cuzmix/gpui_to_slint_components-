# Spec: HotkeyInput

Source: `ely-core/src/forms/hotkey.rs`.

- A tab stop: md control high, md control padding, 8 px gap, items spread apart, radius md, 1 px border-strong (focus color while focused), surface fill, pointer cursor, base text. Role text input named by `label`, valued by the keystroke.
- While focused it records: shows "Press a shortcut…" (fg-subtle). Otherwise the value as a KbdCombo, or "None" (fg-subtle). A keyboard icon sm fg-subtle at the end.
- Every key down is taken (propagation stopped). Bare Escape blurs. Bare Backspace or Delete reports `None`; any other key reports the keystroke (logged), then blurs. Modifier changes are not key downs, so they record nothing.

## Slint mapping
- `HotkeyInput { value, label, recording-text, none-text; value-changed(value) }`, out `recording`. The value is gpui key syntax; cleared is the empty string. `KeyStrokes.unparse` spells the event for `Theme.platform`.
- Tab is taken like any key (gpui's listener stops it before focus moves); focus then leaves with `clear-focus()`, so the field is no trap. The logs are not ported (no logging from `.slint`).
