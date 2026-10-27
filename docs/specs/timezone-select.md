# Spec: TimezoneSelect

Source: `ely-core/src/forms/date/zone.rs` (`show_offset`, `ZONES`, `TimezoneSelect`).

- A Combobox over every zone the system's tz database knows (skipping and logging any that fail to load), sorted by IANA name, each row's value and label its name and its note the offset now: "UTC+09:00", "UTC−05:30" (a true minus west of Greenwich), "UTC+00:00".
- The chosen zone by name; a pick reports the name. Typing filters by name or offset, as Combobox.

## Slint mapping
- `TimeZones.choices` is filled by `wire_zones!` from `ely_slint::dates::zones` (the list loads once; offsets are recomputed each minute and the model replaced only when one changed, where gpui reads them each frame).
- `TimezoneSelect { owner, zone <=>; zone-changed(zone) }`, out `open`, `has-focus`, `text`. Needs `wire_options!`, `wire_fields!`, `wire_toggles!` and an `OptionLayer`, as Combobox (G48, G54).
