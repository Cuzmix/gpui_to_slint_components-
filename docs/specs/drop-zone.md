# Spec: DropZone

Source: `ely-core/src/forms/files.rs` (`DropZone`, `browse`, `target`).

- A column centered, 8 px apart, padded 24 px across and 32 px down, radius lg, 1 px dashed border-strong outline: an upload icon (lg, fg-muted), "Drop files here" (or "Drop a file here" for one), the optional hint (sm, fg-subtle), and an sm Browse… button 8 px further down.
- Files dropped on it, or chosen with Browse (files only, several when `multiple`, confirm "Choose"), are taken when they pass `taken`: something came, only one unless multiple, files and not folders, and each of one of `kinds` by extension in any case (no kinds takes any). Taken files log and go to `on_drop`; refusals log why.
- While files hover it tints as FileInput does: focus outline on selection when it takes them, danger outline on danger-subtle when not.
- No role of its own; the texts and the button are what assistive technology reads.

## Slint mapping
- `DropZone { owner, hint, multiple, kinds; dropped(paths) }`, out `drag-over`, `takes-drag`. `Files.pictures` is ely's `PICTURES`.
- The dashed outline is `DashedOutline`, 2 px dashes and 1 px gaps (G34). The dialog goes through `Paths` (G56); drops are routed by the host (G64).
