# Spec: Checkbox, CheckboxGroup (and the shared check mark)

Source: `ely-core/src/forms/check.rs`.

- `CheckState`: off, on, mixed (a dash: some children are on).
- `check_mark`: check-size square, radius sm, 1 px border: focus while focused, accent while on or mixed, border-strong while off; fill accent when on or mixed, surface when off; dimmed to 0.5 when disabled. Glyph xs in on-accent: check for on, minus for mixed. After a change (not on first show) the fill mixes from the old to the new over motion-fast (linear), and the glyph grows from 0.6 to 1 and fades in, ease-out cubic.
- Checkbox: a tab stop (unless disabled), row 8 px gap, items centered, base text, fg (fg-disabled when disabled). The label sits beside the box, taking the rest of the row; a press anywhere on the row toggles. Mouse down takes no focus. The next value is on unless the box is on (mixed turns on). Logs. Role check box, toggled true/false/mixed, named by label or `name`; "Unavailable" description when disabled.
- CheckboxGroup: a column 8 px apart, a box per choice (label, disabled), on for selected values; a press reports the selection in choice order.

## Slint mapping
- `CheckMark { state, focused, enabled }`, `Checkbox { state <=>, label, name, enabled, unavailable-text; toggled(on) }` out `has-focus`, `CheckboxGroup { choices, selected <=>; selection-changed(selected) }`.
- `state` and `selected` are in-out and take the press at once (G48). Choice order runs in Rust (`Choices.toggled`, `wire_options!`); on-checks use `Toggles.contains` (`wire_toggles!`).
- Gap G57: assistive technology hears mixed as unchecked.
