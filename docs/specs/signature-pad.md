# Spec: SignaturePad

Source: `ely-core/src/forms/signature.rs` (`SignaturePad`, `Stroke`).

- A pad 160 px tall, radius lg, 1 px border-strong on surface. A signing line runs 24 px in from each side and 32 px up: "×" in sm fg-subtle, then, while empty, "Sign here", over a dashed border-strong rule.
- A left press starts a stroke; a drag past 2 px from the press draws it, each point kept inside the pad; the stroke lands when the pen lifts, inside the pad or out. Strokes are drawn in fg, 2 px wide (`signature-ink`), as a quadratic curve through the points' midpoints, with a round dot at each end. The pointer is a crosshair.
- Under the pad, right-aligned 4 px down, a small ghost "Clear" button wipes every stroke; it is disabled while the pad is empty. `on_change` hears the strokes after each stroke and after Clear.

## Slint mapping
- `SignaturePad { strokes <=> [[PenPoint]]; strokes-changed }`, out `drawing`. `strokes` is in-out (G48).
- Growing a stroke, keeping it inside and bending it into path commands run in Rust (`Signatures`, `wire_signatures!`).
- The dashed rule is 2 px dashes with 1 px gaps (G34).
