# Spec: TimePicker, DateTimePicker, DurationPicker (and the time columns)

Source: `ely-core/src/forms/date/time.rs` (`time_columns`, `time_keys`, `nudged`, `TimePicker`, `DateTimePicker`, `DurationPicker`, `parts`).

- Time columns: hours 00–23 and minutes every `step` (00, 05, …), two columns side by side, sm text, a 1 px border-colored line between. Each column scrolls (no bar), at most the list's max height, 4 px padding. Cells sm control tall, 12 px side padding, radius md; the chosen hour and minute accent/on-accent, others fg with a hover fill, pointer cursor. A press sets that part (keeping the other). Whenever the time differs from the one last shown, each column scrolls so its chosen row sits two rows down.
- TimePicker: a picker field (clock icon) showing "07:30" or "Pick a time"; step 5 unless set (must divide the hour); size; disabled. The popup holds the columns inside a focusable spin button named "Time", valued by the time: Up/Down move the time by the step, wrapping around midnight (23:55 + 10 → 00:05); Enter and Escape close. The columns open on the value or midnight. A change reports and stays open.
- DateTimePicker: calendar-days icon; "Sep 5, 2026 09:30" or "Pick a date and time"; never disabled. The popup is a row: the calendar (the value's day chosen, today) and, behind a 1 px border line, the time columns. A day press reports that day at the current time; a time press reports the value's day (else today) at that time. Neither closes; the calendar's Escape does. No time keys.
- DurationPicker: number fields 4 md cells wide, 8 px apart, items centered: hours (0–9999, "h"), minutes (0–59, "m"), and seconds (0–59, "s") when asked; named Hours, Minutes, Seconds. Each change reports the whole duration from the parts.

## Slint mapping
- `CivilTime { hour, minute }` and `TimeMath` (`nudged`, `digits`, `text`) in pure Slint; `TimeColumns { time, step; picked(time) }`.
- `TimePicker { time <=>, chosen <=>, step, placeholder, size, enabled; time-changed }`; `DateTimePicker { date <=> (year 0 none), time <=>, step, placeholder, today, size; moment-changed(date, time) }`; `DurationPicker { value <=> (whole seconds), seconds; duration-changed }`. In-out values (G48); minutes only (seconds of a time are not kept).
