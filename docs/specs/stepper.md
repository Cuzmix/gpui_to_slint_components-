# Spec: Stepper

Source: `ely-core/src/forms/stepper.rs` (`Stepper`).

- A row, items centered, the md control height, 2 px padding inside a 1 px border-strong border, radius md, surface fill.
- A ghost sm icon button (minus) disabled at or below min; the number, at least the md control height wide, centered, base size, fg, formatted with grouping at `precision` places (0 unless set) and a typographic minus; a ghost sm icon button (plus) disabled at or above max.
- A press reports the value plus or minus the step, clamped to min..max. Unbounded (f64 min..max) and step 1 unless set. Panics on min above max or a step that is not positive. Not itself a tab stop: the buttons are.

## Slint mapping
- `Stepper { value <=>, minimum, maximum, step, precision; value-changed(value) }`. In-out value (G48); floats, bounded by ±1e38 (G55). The buttons are named "minus" and "plus" as gpui falls back to the icon names.
- The number comes from `TextFormat.number` (`wire_text_format!`); unwired, Slint's own rendering of the float.
