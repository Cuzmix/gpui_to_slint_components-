# Spec: DatePicker and DateRangePicker (and the picker field)

Source: `ely-core/src/forms/date/picker.rs` (`picker_field`, `Dropdown`, `toggle`, `popped`, `DatePicker`, `DateRangePicker`, `ordered`).

- Picker field: the select's field button with an icon (sm, fg-subtle) then the value or placeholder. Role combo box named by the placeholder, valued by the shown value, "Unavailable" when disabled. A press (mouse down, focus not taken by the press) toggles; closed, Down/Enter/Space open. Opening moves focus into the popup; closing returns it to the trigger. Disabled closes it; leaving both trigger and popup closes it.
- Popup: the option surface under the field, rising in like a list, about nine rows tall for placing; a press outside closes.
- DatePicker: calendar icon; shows the day ("Oct 23, 2026") or "Pick a date"; the popup is a calendar with the picker's min, max and today, the value chosen; a pick reports the day and closes; Escape closes.
- DateRangePicker: calendar-range icon; shows the span (`show_span`) or "Pick dates"; never disabled. Opening clears the pending start and preview. The calendar shows the span from the start to the hovered day (ordered) once a start is set, else the value; it opens on the start, else the value's first. The first pick sets the start; the second reports the ordered span and closes.

## Slint mapping
- `PickerField { icon, shown, placeholder, size, enabled, open; toggled(open) }` and `PickerPopup` (inherits PopupWindow, closes on an outside press). Each picker holds its own popup and focuses its content in the popup's `init`, since Slint forbids reaching into a popup from outside (G61).
- `DatePicker { value <=>, placeholder, minimum, maximum, today, size, enabled; date-changed(date) }`, out `open`, `has-focus`.
- `DateRangePicker { first <=>, last <=>, placeholder, today, size; range-changed(first, last) }`, out `open`, `has-focus`, `start`, `preview`. In-out values (G48); days are `CivilDate` and need `wire_dates!`.
