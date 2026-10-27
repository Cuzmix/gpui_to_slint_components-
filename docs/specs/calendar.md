# Spec: Calendar (and the date foundations)

Source: `ely-core/src/forms/date/calendar.rs` (`Calendar`, `month_grid`, `week_start`), `date/mod.rs` (`show_date`, `show_month`, `show_span`).

- Words: a day "Sep 5, 2026"; a month title "September 2026"; a span naming only what differs ("Sep 3 – 17, 2026", "Sep 28 – Oct 2, 2026", "Dec 30, 2026 – Jan 2, 2027"); weekdays Mo–Su.
- Grid: 42 days, six weeks from the Monday on or before the 1st.
- Box: 8 md cells wide (224 px), 8 px padding, 8 px between header, weekdays and days; radius lg, no fill. A tab stop (unless the picker gives its own focus).
- Header: the month title (pl 4 px, base, semibold, fg), then ghost sm chevron-left/right buttons 2 px apart, tooltips "Previous month"/"Next month": each moves the cursor a month and turns there.
- Weekdays: seven equal columns, xs, fg-subtle, centered. Days: seven columns 2 px apart, md tall, radius md, 1 px border (focus on the cursor while focused; border-strong on today unless chosen; else clear), sm text; today semibold. Chosen (the day, or either end of the span): accent fill, on-accent text. Inside the span, or the hovered week when `week`: active fill. Else fg, fg-subtle outside the month, fg-disabled out of range; open days hover-fill unless chosen or inside, pointer cursor.
- It opens on the anchor, else the chosen day, else the span's start, else today, and re-seeds when that day changes. A turn to another month slides the days in from 12 px on its side and fades them in over motion-base, ease-out cubic.
- Keys: Left/Right a day, Up/Down a week, Page Up/Down a month, Home/End the week's Monday/Sunday — each moves the cursor, turns to its month and reports a hover of it; Enter/Space pick the cursor; Escape goes to the owner's handler when there is one.
- Pressing an open day picks it: cursor there, turn, report. Hovering an open day reports it; leaving the days (by pointer) reports none.
- Role grid named by the month; days grid cells named by their day, selected when chosen, the cursor the active descendant, "Unavailable" out of range.

## Slint mapping
- `CivilDate { year, month, day }` (a year of 0 is none); `Dates` (`today`, `grid`, `shift`, `week-start`) wired by `wire_dates!`, which keeps `today` current each minute; `DateMath` and `DateWords` are pure Slint, the words through `@tr`.
- `Calendar { selected, span-start, span-end, week, minimum, maximum, today, anchor; picked(date), hovered-changed(date), escape() -> EventResult }`, out `has-focus`, `month`, `cursor`, `hovered`.
- Roles: a table of list items, the cursor read as the table's value (G60). A press on a day focuses the calendar (gpui's tracked focus); the header buttons keep their own focus.
