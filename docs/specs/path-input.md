# Spec: PathInput

Source: `ely-core/src/forms/path.rs`.

- An Input (md) over the field's text: prefix icon sm fg-subtle, folder with `directories()`, file otherwise; suffix a sm ghost Button "Browse…".
- Browse opens the system's file dialog (files, or folders with `directories()`; never several; confirm button "Choose") and logs. Cancelled, failed, empty or closed-window answers log and change nothing. The first path chosen goes whole to `on_choose` when set; otherwise it fills the field if it is UTF-8 (logged and dropped if not).

## Slint mapping
- `PathInput { text <=>, owner, directories, fill, placeholder, label, size, enabled, browse-text, prompt-text; edited, accepted, chosen(path) }`, out `has-focus`.
- Browse calls `Paths.choose(PathRequest { owner, directories, multiple: false, prompt })`. `wire_paths!(app, dialog)` hands it to the host's dialog with an `ely_slint::paths::Answer`, which logs as ely does, drops non-UTF-8 paths and sets `Paths.answered`/`answered-owner`/`answers` in the event loop; the field whose `owner` matches takes the first path.
- `fill: false` is ely's `on_choose`: the field is left and only `chosen` hears the path. `chosen` runs with `fill` too.
- Gap G56: the dialog is the host's.
