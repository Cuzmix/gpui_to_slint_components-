# Spec: MonthPicker, QuarterPicker, YearPicker (and the period grid)

Source: `ely-core/src/forms/date/period.rs` (`period_grid`, `period_picker`, `MonthPicker`, `QuarterPicker`, `YearPicker`, `page_of`).

- Period grid: 8 md cells wide, 8 px padding, 8 px gaps, sm text. Header as the calendar's (title base semibold, pl 4 px; ghost sm chevrons 2 px apart, tooltips "Previous page"/"Next page"), each turning the page. Cells in `columns` columns 4 px apart, md tall, radius md, 1 px border (focus on the cursor while focused; border-strong on now unless chosen), chosen accent/on-accent, else fg with a hover fill, pointer cursor. Role grid named by the title; cells grid cells, selected when chosen.
- Keys: Left/Right one, Up/Down a row, clamped to the cells; Page Up/Down turn; Enter/Space pick the cursor; Escape closes. The cursor is seeded by the chosen cell (else 0) and re-seeded when that changes (a turn to another page).
- Period picker: a picker field (calendar icon, never disabled, md) over a popup holding the grid, about six rows tall for placing. The page persists while closed and re-seeds when the seed changes. A pick reports and closes.
- MonthPicker: shows "October 2026" or "Pick a month"; page = a year, step 1, titled by the year; cells Jan–Dec in 3 columns; chosen when the value's year is the page; now is today's month on today's year.
- QuarterPicker: "Q4 2026" or "Pick a quarter"; cells Q1–Q4 in 4 columns; now is today's quarter.
- YearPicker: the year or "Pick a year"; pages of twelve from a multiple of twelve (`page_of`: 2026 → 2016, −5 → −12), titled "2016 – 2027", step 12, 3 columns.

## Slint mapping
- `PeriodGrid { title, cells | count + numbers-from, chosen, now, columns; turned(by), picked(index), escape() }` and `PeriodPicker { shown, placeholder, seed, step, title, cells, count, numbers-from, chosen, now, columns; picked(page, index) }`, out `page`; the owner fills the page from `page`.
- `MonthPicker { year <=>, month <=> (0 none), today; month-changed }`, `QuarterPicker { year <=>, quarter <=> (0 none), today; quarter-changed }`, `YearPicker { year <=>, chosen <=>, today; year-changed }`; `YearPages.page-of`. In-out values (G48). Grids are tables of list items (G60); popups as G61.
