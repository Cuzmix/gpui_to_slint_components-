# Spec: ColorPicker

Source: `ely-core/src/forms/color/picker.rs` (`ColorPicker`, `pointed`), `ely-core/src/forms/color/fields.rs` (`Format`, `format_select`, `typed_fields`).

- A column 12 px apart, padded half a slider thumb (8 px) left, right and top:
  - the plane: `color_plane` (152 px) high, radius md, the pure hue under a white-to-clear horizontal gradient and a clear-to-black vertical one; its thumb at (s, 1 − v);
  - the hue rail: 12 px (¾ thumb), six hue bands, radius sm at the ends; its thumb at h / 360;
  - the alpha rail, unless opaque: a checker under clear-to-solid; its thumb at a;
  - a row 8 px apart: a md ColorSwatch of the color, the format Select (flex) labeled "Color format" with HEX, RGB and HSV, and an EyeDropper;
  - the typed fields: HEX is a mono text field labeled "Color value", at most 9 characters; RGB is three scrub inputs R, G, B (0–255); HSV is H (0–360), S and V (0–100), 4 px apart; numbers are rounded.
- Thumbs: 16 px circles, 2 px on-media border, raised shadow, filled with the solid color (hue rail: the pure hue; alpha rail: the color).
- Pointing: x and y as fractions of the area, clamped; the plane sets s = x, v = 1 − y; the hue rail h = 360·x; the alpha rail a = x. A press jumps; a drag (past 2 px) follows, even outside. Crosshair cursor. The areas are not focusable and have no role.
- The picker keeps its own HSV color, so grays keep their hue. A new incoming value is taken only when it differs from the last color reported (opaque pickers force alpha 1 on both). Every change forces alpha 1 when opaque, then reports the color.
- Hex field: shows the color's hex unless focused. Enter or blur reads it: a color commits (hue held), a misspelling logs and puts the hex back. Blur commits even unchanged text.
- The eye dropper's color commits with the hue held.

## Slint mapping
- `ColorPicker { owner, value <=>, alpha; color-changed(color) }`, out `hsva`, `format`. `value` is in-out (G48).
- The typed fields are `if` elements bound two-way to private parts the picker sets after each change, so they follow the plane and rails and keep typed text while focused.
- `owner` names the format Select's list (`<owner>-format`) and the dropper (`<owner>-dropper`).
- Needs `wire_options!`, `wire_fields!`, `wire_numbers!`, `wire_colors!` and an `OptionLayer`.
- Colors cross as eight-bit `color`s; the hue and the picker's own HSV stay float (G62).
