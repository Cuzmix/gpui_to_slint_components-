# Spec: Radio, RadioGroup (and the shared radio mark)

Source: `ely-core/src/forms/radio.rs`.

- `radio_mark`: check-size circle, 1 px border (focus while focused, accent while on, border-strong off), surface fill, 0.5 opacity when disabled. On: a radio-dot accent dot; after a change it springs open (`motion::spring`) over motion-base.
- Radio: a tab stop unless disabled (or a group's focus), row 8 px gap, base text, fg (fg-disabled disabled), label beside taking the rest; pressing the row or the keyboard click runs `on_select` only while off. Mouse down takes no focus. Role radio button, toggled by `on`, named by the label; "Unavailable" when disabled.
- RadioGroup: panics with no choices. A column 8 px apart, or a row 16 px apart with `horizontal()`. The only Tab stop is the chosen radio if enabled, else the first enabled one (`step` from the last). Up/Left and Down/Right on a focused radio move focus to the previous/next enabled radio, wrapping, and pick it. A pick logs and reports the value. Role radio group.

## Slint mapping
- `RadioMark { on, focused, enabled }`, `Radio { on <=>, label, enabled, unavailable-text; selected() }` out `has-focus`, `RadioGroup { choices, selected <=>, horizontal, unavailable-text; selection-changed(value) }`.
- A group's rows are `RadioRow`s bound to `selected` (a standalone Radio is in-out, G48). Moves run in Rust (`Choices.step`, `Choices.index-of`, `wire_options!`). An empty group shows nothing. The group is a groupbox for assistive technology (G51).
