# Spec: DragGhost

Source: `ely-core/src/primitives/ghost.rs`.

- A row: optional icon (sm, `fg_muted`), 6 px gap, title (text sm, `fg`), items centered.
- Box: padding 8 × 4 px, radius md, fill `overlay`, 1 px border in `border`, floating elevation. Hugs its content.
- GPUI draws it as the drag preview; the drag system positions it at the pointer. It has no events.

Slint: `DragGhost { title; icon; }`. The owner places it (for example at the TouchArea's `mouse-x`/`mouse-y` while dragging).
