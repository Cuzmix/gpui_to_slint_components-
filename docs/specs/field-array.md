# Spec: FieldArray, KeyValueInput, ListInput

Source: `ely-core/src/forms/pairs.rs`.

- FieldArray: a column 8 px apart: each owner-built row (filling, then a sm IconButton minus, tooltip "Remove", reporting the row's index and logging), then a sm ghost Button with a plus icon, "Add" unless set (logs, runs `on_add`).
- KeyValueInput: seeded pairs once; each row two sm Inputs ("Key", "Value") filling, 8 px apart; "Add pair" adds an empty row and focuses its key; removing drops the row; every change reports all pairs.
- ListInput: the same with one sm Input per row, placeholder "Item" unless set, "Add item".

## Slint mapping
- `FieldArray { add-text; add() }` with `FieldArrayRow { index; remove(index) }` children (the owner's rows; the remove button is the row's).
- `KeyValueInput { pairs: [KeyValue] (in-out); pairs-changed }`, `ListInput { items (in-out), placeholder; items-changed }`. Growing and shrinking the list runs in Rust (`FieldRows`, `wire_rows!`); a new list rebuilds the rows, and the added row's first field takes focus as it is built.
