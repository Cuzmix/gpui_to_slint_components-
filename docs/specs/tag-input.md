# Spec: TagInput

Source: `ely-core/src/forms/tags.rs` (with `combobox.rs` `matching`, `select.rs` `Picker`).

- Tags are the owner's; each change reports the whole new list (logs). Placeholder "Add a tag".
- A wrapping row, items centered, 4 px gap, at least a md control tall, 4 px sides and 2 px above and below, radius md, 1 px border (focus while focused, danger while a typed word fails the check after focus left, else border-strong), surface fill, base text. A press anywhere focuses the field.
- Chips: Tag with a remove button, labelled by the suggestion row holding its value (else the value), danger tone when it fails the check; removing reports the list without it. The field: fills, at least half a pane-min wide, 4 px sides.
- Adding: Enter, or a typed comma outside double quotes (backslash escapes inside quotes): the text trimmed, a trailing comma dropped, trimmed again; blank or already present adds nothing; the field clears either way.
- Suggestions: rows whose label or note holds the typed text (ignoring case), less existing tags, while the text is not empty, focused and not dismissed. Highlight starts at 0 on opening; Up/Down step (skipping disabled, wrapping); Enter or a press adds the row's value. Escape dismisses until the next change. Backspace in the empty field drops the last tag. Every change reports the typed words.

## Slint mapping
- `TagInput { tags, placeholder, label, suggestions: [Choice], owner; pure check(tag) -> bool (default true) }`, `tags-changed([string])`, `typing(text)`; out `typed`, `open`, `highlighted`, `unaccepted`.
- List edits, matching and the comma rule run in Rust (`Choices`, `wire_options!`); the suggestion list needs an `OptionLayer` (G54).
