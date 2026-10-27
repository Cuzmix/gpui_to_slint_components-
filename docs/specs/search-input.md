# Spec: SearchInput, and the option list (Choice, option_row, surface, float, Popup)

Source: `ely-core/src/forms/search.rs`, `ely-core/src/forms/options.rs`.

## Option list
- Choice: value, label, icon, avatar (initials), note, disabled, depth.
- Row: role list box option, named by its label, selected when checked, active descendant while highlighted; disabled rows say "Unavailable". 8 px gap, 8 px sides, 4 px top and bottom, radius md; fg (fg-disabled when disabled); hover fill while highlighted or hovered unless disabled; pointer unless disabled. A check column (sm icon square, a check when on) when the list checks; indent `tree-indent × depth`; icon sm fg-muted; avatar xs; label ellipsized, filling; note fg-subtle at the end. A press picks and keeps focus where it was (mouse down, default prevented).
- Surface: radius lg, overlay fill, 1 px border, floating shadow, text sm.
- Float: under the anchor, or over it when the guessed height (a md row per option plus one, at most `list-max-height`) does not fit below and above has more room; snapped inside the window; rises in over FAST, ease-out cubic: opacity 0 to 1 and an 8 px gap closing to 4 px.
- Popup: at least the anchor's width (256 px when the anchor has none), at most the window less 24 px a side, at most `list-max-height` tall, scrolling; 4 px padding; an optional dismiss on presses outside.
- `step` moves to the next enabled row, wrapping; stays put when all are disabled.

## SearchInput
- An Input with a sm magnifier in fg-subtle, clearable. While focused, empty and with history, the history floats under it (no row highlighted). A pick fills the field (a changed event), logs, then runs `on_pick`.

## Slint mapping
- `OptionLayer` draws the one open list; the window puts it last, filling it. Fields hold an `OptionAnchor` (owner id, open, rows or plain labels, highlighted, checks, checked; `picked(index)`), which hands them over through the `OptionFloat` global. A PopupWindow would take keyboard focus from the field (gap G54).
- `Choices.step` and `Choices.plain` run in Rust (`wire_options!`); Slint cannot walk an array. Checked values use `Toggles.contains` (`wire_toggles!`).
- Avatar rows draw once Avatar lands (chapter 12).
