# Spec: Viewport (Transform), FloatingPanel

Source: `ely-core/src/layout/viewport.rs`, `floating.rs`; stories in `ely-gallery` `pages/layout/systems.rs`.

## Viewport
- Fills its parent and clips. Transform: `local = content × scale + offset`, starting at offset 0, scale 1; `invert` is the reverse.
- Wheel: with the secondary modifier (⌘ on macOS, Ctrl elsewhere) or Ctrl held, zoom by `exp(−Δy × 0.01)` about the pointer, the scale clamped to the zoom range (0.1 to 8 unless set; an empty range panics); otherwise the offset moves by the wheel delta. The wheel stops there.
- A left press grabs; moving with the button held sets offset = start + (pointer − press); release, inside or out, lets go.

## FloatingPanel
- Raised over the page at a window position; the position is seeded state (a new start replaces a dragged one), kept so that `control_height(Lg)` (32 px) stays inside the window: x in [keep − window width, window width − keep], y in [0, window height − keep].
- Panel: a column that clips, radius lg, `border`, `overlay` fill, `Floating` shadow. Title bar: control height md (28 px) with a `border` hairline at the bottom, 12 px sides, grab cursor, the title sm semibold `fg_muted`. Body: 12 px padding.
- Dragging the bar: the first move's position anchors; each move sets corner = clamp(start + (pointer − anchor)) and reports it.

## Slint
- Viewport exposes `offset-x/y` and `scale` (two-way), `apply-x/y`, `invert-x/y` and `zoom-at`; children place themselves with them. Slint reports ⌘ as control on macOS and Ctrl as meta, so the zoom test reads both there. An empty zoom range is kept usable (least above 0, most at least the least) instead of panicking. Wheel deltas arrive in pixels (G31 applies to how the platform turns lines into pixels).
- FloatingPanel fills its parent and reads it as the window: put it last in the window (G12). `panel-width/height` stand for gpui's `.w()`/`.h()` on the panel. A press on the panel stays on it.
