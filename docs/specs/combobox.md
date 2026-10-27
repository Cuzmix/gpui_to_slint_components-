# Spec: Combobox (and Autocomplete, its `free` form)

Source: `ely-core/src/forms/combobox.rs`.

- An Input (size md by default) with a chevrons-up-down suffix (xs, fg-subtle).
- The field shows the chosen label when `selected` changes while unfocused; with nothing chosen it shows empty unless `free`. On blur (not free) the chosen label (or empty) is put back if the text differs (logs).
- Rows: choices whose label or note holds the text, trimmed, ignoring case; every row while the text is the chosen label. Open while focused, not dismissed, with rows. On opening the cursor goes to the chosen row if it is there and enabled; the cursor otherwise stays, or falls to the first enabled row.
- Up/Down walk past disabled rows (wrapping) while open; Down while closed and dismissed shows the list again. Enter picks. Escape dismisses until the text changes. A pick sets the text to the row's label, hides the list and reports the value (logs). A floating list with a check column checking the choice; no outside-press handling (blur closes it).

## Slint mapping
- `Combobox { owner, choices, selected <=>, free, placeholder, label, size, enabled; selection-changed(value), edited(text), accepted }`, out `has-focus`, `text`, `open`, `highlighted`. `selected` is in-out (G48).
- Matching runs in Rust (`Choices.matching`, `Choices.index-of`, `wire_options!`), only while focused. The field's text is a binding to the chosen label until the first edit, then set by picks, blur and new choices. The list goes through `OptionAnchor`/`OptionLayer` (G54).
