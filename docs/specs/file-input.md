# Spec: FileInput (and the file foundations)

Source: `ely-core/src/forms/files.rs` (`FileInput`, `dropped`, `taken`, `hovering`, `file_name`, `PICTURES`).

- A field (field button, md): a file icon (sm, fg-subtle), the value or the placeholder ("Choose a file…", or "Choose files…" when it takes several), and a clear button (x, sm, ghost, tooltip "Clear") while it holds files. The value is the file's name (a root shows its whole path), or the first name and "+N" for the rest.
- A click, or Enter or Space (gpui's keyboard click), opens the system's file dialog: files only, several when `multiple`, confirm "Choose". What it brings becomes the paths. Clear empties them. Each change logs the count and reports the paths.
- Files dropped on it are taken when they pass `dropped`: something came, only one unless multiple, files and not folders; otherwise the drop is logged and refused. While files hover, the field tints: focus border on selection when it takes them, danger border on danger-subtle when not.
- A button named by the placeholder, its value the shown text; a Tab stop. Panics holding several files without `multiple`.

## Slint mapping
- `files.slint`: `Files` (Rust via `wire_files!`: `refusal`, `picture-refusal`, `name`, logging, the routed `drag`), `FileDrops` (tints), `DashedOutline` (G34).
- `FileInput { owner, paths <=>, multiple; paths-changed(paths) }`, out `has-focus`, `drag-over`, `takes-drag`. `paths` is in-out (G48); several paths without `multiple` show as given.
- The dialog goes through `Paths` (G56); drops are routed by the host to the `owner` (G64). FieldButton gained `clicked`, `key-released` and the drop tint for this.
