# Spec: CascadeSelect

Source: `ely-core/src/forms/cascade.rs` (`Cascade`, `Cascader`, `columns`, `trail_of`, `fit_trail`, `nodes`).

- Field: the select's field button (`field_button`, never disabled) showing the labels along the chosen path joined by " / ", or the placeholder ("Choose…"), then a chevrons-up-down icon (xs, fg-subtle). Panics with no roots.
- Role combo box named by the placeholder, valued by the shown path when one is chosen.
- State: open, and a trail (one row index per open column), cut back each render to rows that exist (never empty). Leaving the field closes.
- A mouse click toggles, opening at the chosen path's trail, else `[0]`.
- Keys, closed: Down, Up, Enter, Space open at that start. Open, at the trail's last level: Down/Up wrap within the column; Right on a branch steps in at 0; Left above the roots steps out; Enter/Space on a leaf picks, on a branch steps in at 0; Escape closes. Other keys pass.
- Columns: one per trail step with children, after the roots. Each column is three quarters of the tooltip's max width wide, at most the list's max height tall, scrolling, 4 px padding, a 1 px border-colored line on the left past the first. Rows are option rows lit where the trail passes; a branch ends with a chevron-right (xs, fg-subtle). Hovering a row sets the trail to it; clicking a leaf picks.
- The columns float on the option surface (radius lg, overlay, hairline, floating shadow) under the field (or over it, guessed from the longest column), rising in like any list. A press outside closes.
- A pick closes, keeps the trail and reports the values from the root to the leaf.

## Slint mapping
- `CascadeNode { value, label, parent }` flat (G58); the `Cascades` global (`columns`, `trail-of`, `fit`, `path`, `shown`, `step-to`, `deepest`) answered by `wire_cascades!` (`ely_slint::cascade`). `Choice.branch` marks rows with children, drawn as the chevron by `OptionRow`.
- `CascadeSelect { owner, nodes, selected <=>, placeholder, size; selection-changed(path) }`, out `has-focus`, `open`, `trail`. `selected` is in-out (G48).
- The columns go through `OptionAnchor`/`OptionLayer` (G54), which gained a columns mode: `OptionFloat.columns`, `trail`, `deepest`, and hover and press reports through `reached(level, index, pick)`.
- No roots shows the placeholder and never opens (gpui panics).
