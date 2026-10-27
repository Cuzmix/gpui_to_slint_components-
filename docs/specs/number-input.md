# Spec: NumberInput, ScrubInput

Source: `ely-core/src/forms/number.rs`, `number/{value,scrub}.rs`.

- Value f64, default range f64::MIN..=f64::MAX, step 1 (panics unless positive), precision 0; range panics when empty. Prefix symbol (fg-subtle); `currency(code)` sets the symbol and its minor units as precision; `percent` adds "%" after; any suffix. Label for assistive technology; `blank(placeholder)` shows no number until typed in.
- Typing filters to digits, `.`, `,`, `-` and the typographic minus. While focused, each typed number that is already settled and differs from the value reports `on_change` (logs "typed"). Enter or blur: settle the typed number (or the value when the text is none), show it, report change and commit; an empty blank field commits nothing. Up and Down step by `step` and commit. A new owner value replaces the text unless the field is being edited (then once focus leaves).
- Settle: round to the precision, clamp to the rounded numbers inside the range (bounds kept when float error alone puts them off the grid).
- Layout: [label] gap 8, then the Input: prefix (grip: chevrons-up-down xs fg-subtle, east-west cursor, and the symbol, 4 px apart), the text (holding room for the wider of the formatted ends when both are set), suffix (any suffix, then the stepper, 4 px apart).
- Stepper: full height, a hairline on its left in the border color; two halves each an lg icon wide, chevron xs fg-muted; enabled while the current number is below the max (up) or above the min (down): pointer, hover fill, presses keep focus; otherwise 40 % opacity.
- Scrub: dragging the grip (or ScrubInput's label: min width a md control, sm fg-muted, no wrap) moves the number from where the drag began by whole notches of `handle-hit` (6 px), truncated toward zero, one step each, committing on every move.

## Slint mapping
- `NumberInput { value (in-out), minimum, maximum, step, precision, prefix, currency, percent, suffix, label, blank, size }`, `value-changed(value)`, `committed(value)`; out `text`, `current`, `can-step-up`, `can-step-down`. ScrubInput inherits it; its `label` is the scrub label.
- Parsing, settling and showing run in Rust (`Numbers`, `wire_numbers!`, with `wire_fields!` for the `number` filter). Limits default to ±1e38 (`Numbers.unbounded`); Slint floats are single precision (gap G55). An empty range logs and settles low instead of panicking.
