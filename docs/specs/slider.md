# Spec: Slider and RangeSlider (and the shared track)

Source: `ely-core/src/forms/slider.rs` (`fraction`, `value_at`, `keyed`, `along`, `Track`, `Slider`, `RangeSlider`).

- Math: a value's fraction of min..max, clamped 0..1; the value at a fraction, snapped to the step grid from min and clamped; keys: Left/Down −step, Right/Up +step, Page Down −10 steps, Page Up +10 steps, Home min, End max, clamped to min..max.
- Track: horizontal, the slider-thumb tall with half a thumb of padding at each side (vertical: a thumb wide, half a thumb above and below, the parent gives the height); half opacity when disabled. A full-round rail (slider-track thick, border color) through the middle, an accent fill from the start (or the low thumb) to the value (or high thumb). Thumbs: slider-thumb round, surface fill, 1 px border (focus while focused, else border-strong), raised shadow, centered on their fraction; the high thumb drawn above the low; pointer cursor.
- Press inside the inner track: the value there, the nearer thumb (the high one only when strictly nearer) grabbed and focused, committed. A press on a thumb focuses it and drags it; a press on the track drags the grabbed thumb. A drag commits the value under the pointer, clamped to the track.
- Thumb keys commit `keyed` values. Each thumb is a tab stop unless disabled; role slider named by the label (a range's thumbs "{name}, from" and "{name}, to"), valued, bounded by min (or the low thumb) and max (or the high thumb), stepped, oriented, "Unavailable" when disabled.
- Slider: 0..100 step 1; reports a new value only when it differs. RangeSlider: the low thumb stops at high and the high at low; reports the pair only when it changes. Panics on an empty range, a step that is not positive, or low above high.

## Slint mapping
- `SliderMath` (pure Slint: `fraction`, `value-at`, `keyed` → `SliderKey`), `SliderTrack { range, low, high, minimum, maximum, step, vertical, enabled, low-label, high-label; commit(index, value) }`; thumb 0 is the low one, 1 the high or only one.
- `Slider { value <=>, minimum, maximum, step, label, vertical, enabled; value-changed(value) }`; `RangeSlider { low <=>, high <=>, …; range-changed(low, high) }`. In-out values (G48); floats (G55).
- One TouchArea over the track takes every press and finds the thumb under it, so the thumbs' FocusScopes only take keys. Focusing the slider focuses its high (or only) thumb. Horizontal sliders stretch across their row (gpui's take the parent's cross size in a column). No orientation for assistive technology (G59).
