# Spec: RelativeDatePicker

Source: `ely-core/src/forms/date/relative.rs` (`RelativeRange`, `RelativeDatePicker`).

- Spans named from today, in menu order: Today, Yesterday, Last 7 days (today included), Last 30 days, This month (first to last day), Last month, This quarter (all three months), This year (Jan 1 – Dec 31). Keys: today, yesterday, last-7-days, last-30-days, this-month, last-month, this-quarter, this-year.
- A Select of those spans, each row's note the days it covers (`show_span`), placeholder "Any time", size; the chosen span selected by its key. A pick reports the span and its first and last day counted from today (the system's date unless set).

## Slint mapping
- `RelativeRange` enum with a leading `none` for no choice; `DateSpan { first, last }`; `RelativeRanges` (`all`, `keys`, `labels`, `span`, `choice`, `index-of`, `index-of-key`, `key-of`, `last-of-month`) in Slint over `Dates.shift`.
- `RelativeDatePicker { owner, range <=>, today, size; range-changed(range, first, last) }`, out `open`, `has-focus`. Needs `wire_dates!`, `wire_options!`, `wire_toggles!` and an `OptionLayer`, as Select (G48, G54).
