# Spec: UploadList

Source: `ely-core/src/forms/upload.rs` (`UploadList`, `Upload`, `UploadState`).

- A column of rows in base text. Each row: 12 px across and 8 px down, 12 px apart, items centered:
  - an icon (md): circle-check in success when done, circle-alert in danger when failed, file-text in fg-muted while uploading;
  - the name (medium weight, ellipsized, flexing) and a note (xs, tabular) on one line 8 px apart: the percent while uploading, the size (decimal units) when done, both fg-subtle; the reason in danger when failed;
  - while uploading, a progress bar of the share under them, 4 px down;
  - a button (sm, ghost) when the owner handles it: x "Cancel" while uploading, rotate-cw "Retry" when failed, x "Remove" when done; each reports the row.
- Panics on a share outside 0..1.

## Slint mapping
- `UploadList { uploads: [Upload], can-cancel, can-retry, can-remove; cancel(index), retry(index), remove(index) }`. `Upload { name, bytes, state, share, reason }` flattens gpui's `UploadState` payloads into `share` and `reason`.
- Slint cannot tell whether a callback is set, so each button shows when its `can-*` is true.
- Sizes are `float`s (G55) read by `TextFormat.file-size` (`wire_text_format!`); digits are proportional (G20). A share outside 0..1 is clamped by the progress bar.
