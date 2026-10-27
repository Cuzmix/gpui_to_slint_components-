# Spec: Knob

Source: `ely-core/src/forms/knob.rs` (`Knob`, `arc`, `at_angle`), with the slider's `fraction`, `value_at` and `keyed`.

- A round dial of the knob diameter (48 px): surface fill, 1 px border (focus while focused, else border), raised shadow; half opacity when disabled. 0..100 step 1 unless set.
- Drawn inside the border, stroke the knob arc width (3 px), radius half the inner side less 1.5 strokes: a rail arc (border color) clockwise from 135° through 270°; the value's arc (accent) from 135° to its fraction of the sweep, drawn only above zero; a tick (fg) at the value's angle from 0.3 to 0.62 of the radius.
- Drag (ns-resize cursor): the first move anchors the pointer's y and the value; each move after sets the value at the anchored fraction plus the rise over four diameters, snapped and clamped. Keys as a slider's. Reports only a changed value. A tab stop unless disabled; a press focuses.
- Role slider named by the label, valued, bounded, stepped. Panics on an empty range or a step that is not positive.

## Slint mapping
- `Knob { value <=>, minimum, maximum, step, label, enabled; value-changed(value) }`, out `has-focus`. In-out value (G48); floats (G55). The arcs are `Path`s whose SVG commands are built from the value; increment and decrement actions step it.
