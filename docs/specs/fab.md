# Spec: FloatingActionButton

Source: `ely-core/src/buttons/fab.rs`.

- Props: icon, label (optional), tooltip (optional), on_click.
- 56 px tall (`fab_size`); round (56 px wide) without a label; with one, a pill: 20 px left padding, 24 px right, inside a 1 px border.
- Icon lg in on-accent; label md, medium weight, on-accent; 8 px apart, centered.
- Fill accent; hover accent-hover and the shadow lifts from floating to modal. No pressed fill.
- 1 px transparent border that turns focus-colored while focused.
- Role button, name label else tooltip. Pointer cursor, Tab stop, gpui click keys; a mouse press takes no focus. Tooltip when given.
- Entrance: over SLOW (320 ms), ease-out-cubic, opacity t and top margin 8 px × (1 − t).

## Slint mapping
- `FloatingActionButton { icon; text; tooltip; clicked() }`; out `current-fill`, `current-elevation`, `entrance`.
- The face is an `Elevated` drawn `8 px × (1 − t)` lower; the button's own box stays put (gap G49).
