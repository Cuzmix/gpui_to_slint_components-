# Spec: ColorSwatch, ColorPalette (and the color foundations)

Source: `ely-core/src/forms/color/swatch.rs` (`ColorSwatch`, `ColorPalette`, `checker`, `dark_cells`), `ely-core/src/forms/color/mod.rs` (`Hsva`, `parse_hex`, `hex`).

- Foundations: `Hsva` (hue in degrees; saturation, value, alpha 0..1) converts to RGB by the six hue sectors; RGB to HSV keeps a given hue for grays. `parse_hex` reads 3, 4, 6 or 8 hex digits with or without `#`, else fails with `"{text:?} is not a hex color"`. `hex` spells `#rrggbb`, adding alpha only when it is not opaque.
- Checker: a light (`surface`) box with `radius` corners (at most half the shorter side); dark (`active`) cells at least a quarter of the shorter side and the radius, odd counts each way, dark where row + column is odd, so no dark cell reaches a rounded corner.
- ColorSwatch: a square of the control height of its size (sm 24, md 28), radius md, 1 px border: focus while focused, accent when selected, else border. Selected swatches pad 2 px inside the border. The patch has radius sm, over a checker when see-through.
- With a click handler it is a button: named by its tooltip, else its hex; reports selected; a Tab stop; pointer cursor; a press never takes focus. A tooltip shows on hover when given.
- ColorPalette: swatches (sm) in a wrapping row 6 px apart, at most `columns` (8) wide. Each is named by its color's name (tooltip and button name); the swatch whose color equals the chosen one is selected; a press reports its color (logged).

## Slint mapping
- `colors.slint`: `Hsva`, `ColorMath` (`color-of`, `hsva-of`, `hue`, `hex`, pure Slint), `Colors` (`parse-hex`, Rust via `wire_colors!`), `Checker`.
- `ColorSwatch { color, size, selected, tooltip, clickable; clicked }`, out `has-focus`, `tooltip-shown`. Slint fixes an element's role, so the button (role, focus, Enter/Space) is an element present only while `clickable`.
- `ColorPalette { colors: [NamedColor], selected <=>, chosen <=>, columns; selection-changed(color) }`. `selected` and `chosen` are in-out (G48); `chosen` false is gpui's `None`. Wrapping is a `FlexboxLayout`. A zero `columns` acts as one (gpui panics).
- Colors are eight bits a channel (G62).
