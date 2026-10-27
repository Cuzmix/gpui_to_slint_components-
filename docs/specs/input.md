# Spec: TextField (TextInput), Input, PasswordInput

Source: `ely-core/src/forms/text/{mod,actions,element,edit}.rs`, `ely-core/src/forms/input.rs`.

## TextInput (the editor)
- One line (default) or several: `multi_line(min, max)` grows from `min` rows to `max`, then scrolls; panics unless 0 < min <= max. Line height is the window's (gpui default phi, 1.618034 × the font size).
- Placeholder in fg-subtle while empty; the label (else the form field's name, else the placeholder) names it for assistive technology, role text input; its value is read only while unmasked.
- Masked: one bullet per grapheme, kept off the clipboard. Filter drops rejected characters; max-len caps characters (kept ones counted first). One-line text turns newlines into spaces; several lines normalize CRLF. A `fit` reshapes every edit (text, replaced range, typed text in; text and caret out).
- Selection fill: theme selection. Caret: focus color, `caret-width` (1.5 px, snapped), blinks only while focused. Scrolls to keep the caret in view.
- Keys: the usual editing set (characters, words with alt/ctrl, home/end, cmd-left/right on macOS, select all, copy, cut, paste, undo, redo with shift-secondary-z and ctrl-y off macOS). Enter submits one line and breaks a line in several; shift-enter breaks; secondary-enter submits (or the form). Events: changed, submit, focus, blur. Disabled: no input, no Tab stop.

## Input (the frame)
- A row, 8 px gap, control padding per side; one line: items centered, control height; several: items at the start, 6 px top and bottom. Radius md, 1 px border: danger if invalid, focus while focused, border-strong otherwise. Fill sunken when disabled, else surface. Text size by control size (sm → sm, md → base, lg → md); fg, fg-disabled when disabled.
- A press anywhere focuses the text. Prefix before, suffix after. `least(text)`: a zero-height strut holding room for `text` whole (control padding and 1 px each side).
- Clearable: a sm IconButton circle-x, tooltip "Clear", while there is text and it is enabled; it empties the text and focuses it.

## PasswordInput
- An Input with a lock icon (sm, fg-subtle) before and a sm IconButton after: eye while masked ("Show password"), eye-off while shown ("Hide password"); a press flips masking and logs.

## Slint mapping
- `TextField` wraps the builtin `TextInput`: `in-out text`, placeholder, label, masked (`input-type: password`), `rows-min`/`rows-max` (0: one line), `max-len`, `fit-kind`, enabled; `edited(text)`, `accepted()`, `has-focus`. Line height through `line-height-factor`.
- Filters, max-len and fits run in Rust through the `TextFields` global (`ely_slint::fields`, `wire_fields!`): Slint cannot walk the characters of a string. Kinds are named; the host registers its own.
- `Input` has `prefix-icon` and puts its `@children` after the text as the suffix (one children slot, gap G47). PasswordInput inherits Input.
- The builtin editor's undo keeps edits a filter rejected, its caret blinks on its own timer, and its key set is Slint's (gap G52). Spans cannot be colored in a TextInput (G53).
