# Spec: WeekPicker

Source: `ely-core/src/forms/date/week.rs` (`WeekPicker`, `monday_of`).

- A picker field (calendar-range icon, md, never disabled) showing "Week 43, 2026" or "Pick a week".
- The popup is a calendar in week mode (the hovered week shades) with today; a chosen week marks its Monday to Sunday as a span. A pick reports the ISO year and week of the day and closes; Escape closes.
- `monday_of`: the Monday of an ISO week (2026-W39 → Sep 21, 2026; 2027-W1 → Jan 4, 2027); panics on a week that does not exist.

## Slint mapping
- `WeekPicker { year <=>, week <=> (0 none), today; week-changed(year, week) }`, out `open`, `has-focus`. In-out values (G48).
- `Dates.monday-of` and `Dates.iso-week` (`IsoWeek { year, week }`) in Rust; a week that does not exist shows no span instead of panicking.
