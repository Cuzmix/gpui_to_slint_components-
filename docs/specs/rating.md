# Spec: Rating (and the star shape)

Source: `ely-core/src/forms/rating.rs` (`Rating`), `ely-core/src/data_display/stars.rs` (`star`).

- Star: a five-point star centered in its box; the stroke is a twelfth of the side, tips at half the side less the stroke, notches at half that. Filled stars are filled and stroked; empty ones stroked.
- Rating: `max` (5) stars of icon size lg, 2 px apart, 2 px padding, inside a 1 px border (focus while focused, else clear), radius sm; half opacity when disabled. Filled stars (up to the hovered star, else the value) in accent; the rest in border-strong.
- Hovering a star previews up to it; leaving the row ends the preview. Pressing star n reports n, or 0 when n is the value. A press does not take focus (prevented mouse down). Pointer cursor.
- Keys while focused: Left/Down one less (not below 0), Right/Up one more (not above max), Home 0, End max; reported only when it changes. A tab stop unless disabled.
- Role slider named by `label`, value 0..max in steps of 1. Panics on a value above max or a max of zero.

## Slint mapping
- `StarShape { filled, color }` is a `Path` on a 24-unit square, its ten points worked out once.
- `Rating { value <=>, max, label, enabled; value-changed(value) }`, out `has-focus`, `hovered`. `value` is in-out (G48). Increment and decrement actions step it.
- A value above max fills every star; a max under one draws one (gpui panics).
