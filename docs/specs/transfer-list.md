# Spec: TransferList

Source: `ely-core/src/forms/transfer.rs`.

- A row 12 px apart: the left side (choices not chosen), the buttons, the right side (chosen), sides taking equal room.
- A side: a column 8 px apart: "Title · count" (sm, fg-muted; titles "Available" and "Chosen" unless set), then a list-max-height frame holding a multiple ListBox of its rows (nothing while empty), filling it. The ListBox's selection is the side's marks.
- Buttons: a column centered 8 px apart: outline IconButtons chevron-right ("Move to chosen", disabled with no left marks) and chevron-left ("Move back", disabled with no right marks). A move puts the marked values in (or takes them out), in choice order, clears both sides' marks, logs, and reports the chosen values.

## Slint mapping
- `TransferList { choices, chosen <=>, available-title, chosen-title, add-text, remove-text; chosen-changed(chosen) }`. `chosen` is in-out (G48).
- Sides, moves and marks run in Rust (`Choices.matching` with no query for the left rows, `among`, `moved`; `wire_options!`, `wire_toggles!`).
