# Spec: SplitPane (ResizablePanel / ResizablePanelGroup), ResizeHandle (resize_handle, resize_edge)

Source: `ely-core/src/layout/split.rs`, `seeded.rs`; stories in `ely-gallery` `pages/layout/regions.rs`.

## ResizeHandle
- A box `handle_hit` (6 px) across the split, full length along it; a 1 px line centered in it, `border` at rest (`resize_edge`: transparent), `focus` while the handle is hovered. Cursor: resize left-right for a horizontal split, up-down for a vertical one. It never grows or shrinks (`flex_none`).

## SplitPane
- Two or more panes (panics on fewer), with a handle between each pair; fills its parent; a row (horizontal) or column (vertical).
- Shares: `sizes` normalized; none gives even shares; the wrong count or weights that are negative, not finite, or sum to zero log an error and give even shares. Shares are seeded state: drags change them until the owner passes a different `sizes`.
- Each pane: flex-basis 0, grow by its share, clips (so it may shrink to nothing).
- Dragging divider i: the first move's position is the anchor; each move sets `shift(start, i, (along − anchor) / extent, min / extent)` where `extent` is the whole split's length (handles included) and `start` the shares when the drag began: the pair keeps its sum, the left share is clamped to [floor, pair − floor] with floor = min(min / extent, pair / 2). `on_resize` gets the shares after each step.
- Edges snap to device pixels.

## Slint
- The owner lays out SplitCells and ResizeHandles in order and wires each by index: a cell binds `share: split.share(i)`, a handle calls `split.grab(i)` and `split.drag(i, along)` (`along` in window coordinates, which the handle reports). `count` names the number of panes. `resized` reports each step; `shares` holds the current ones.
- Normalizing and shifting loop over the panes, so they come from `LayoutMath.split-shares` and `split-shift` (Rust, the same functions as gpui's `check_weights` and `shift`). Unwired, the weights are the panes' stretch factors as given (negative counts as 0) and handles do not move.
- The split is a non-wrapping FlexboxLayout that stretches its items by their shares from a basis of 0. A new `sizes` value (a changed property) replaces the dragged shares.
- ResizablePanel and ResizablePanelGroup are SplitPane (the task list's alias); ThreeColumnLayout is ColumnShell (chapter 4).
