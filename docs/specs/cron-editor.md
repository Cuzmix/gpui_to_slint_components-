# Spec: CronEditor (and the cron rule)

Source: `ely-core/src/forms/date/cron.rs` (`CronEditor`, `shown`), `date/rule.rs` (`CronRule`, `weekday_words`).

- Rule: five whitespace-separated fields (minute 0–59, hour 0–23, day 1–31, month 1–12 or JAN–DEC, weekday 0–7 or SUN–SAT, 7 = Sunday), each `*`, values, ranges, lists and `/` steps; errors say what is wrong ("a rule has five fields, not 3", "minute 61 is outside 0 to 59", "hour range 5-2 runs backwards", "minute step \"0\" must be a positive number"). Days follow Cronie: a day field starting with `*` makes both day fields apply, otherwise either. Next runs look up to five years ahead. Words: "At 09:00, Monday through Friday", "Every 15 minutes", runs of three or more as "a through b", lists "a, b and c".
- Editor: a column 8 px apart: a wrapping row of outline sm preset buttons (Hourly `0 * * * *`, Daily at 9:00 `0 9 * * *`, Weekdays at 9:00 `0 9 * * 1-5`, Mondays at 9:00 `0 9 * * 1`, Monthly `0 0 1 * *`) that set the field; the field in the mono face, invalid when the rule is wrong; then sm text: blank, "Five fields: minute, hour, day of month, month, weekday." in fg-subtle; wrong, the problem in danger; a rule, its words in fg and a line in fg-muted "Next: Mon Sep 28, 09:30 · …" or "No run in the next five years.". Three runs unless set; runs count from the system clock (shown in its zone, "UTC" added in the unknown zone) unless a moment is given.

## Slint mapping
- `ely_slint::cron` is the rule ported whole; `Crons.read` (wired by `wire_crons!`) returns a `CronReading { state, words, runs, any }`; `Crons.minute` ticks each minute so the reading follows the clock.
- `CronEditor { rule <=>, now-date, now-time, runs, label; rule-changed(rule) }`, out `has-focus`, `reading`. The field is named "Cron rule" for assistive technology (gpui's field has no name). Needs `wire_fields!`.
