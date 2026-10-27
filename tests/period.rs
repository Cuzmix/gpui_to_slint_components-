// MonthPicker, QuarterPicker and YearPicker, after ely-core/src/forms/date/period.rs.
#[macro_use]
mod common;
use common::*;
use slint::ComponentHandle;
use slint::platform::Key;

slint::slint! {
    import { MonthPicker, QuarterPicker, YearPicker, YearPages, CivilDate, IsoWeek, Dates } from "ui/index.slint";
    export { CivilDate, IsoWeek, Dates }
    export component Harness inherits Window {
        width: 400px;
        height: 500px;
        in-out property <int> year: 2026;
        in-out property <int> month: 10;
        in-out property <int> q-year;
        in-out property <int> quarter;
        in-out property <int> pick-year;
        in-out property <bool> picked;
        in-out property <int> changes;
        out property <bool> month-open: months.open;
        out property <bool> quarter-open: quarters.open;
        out property <bool> year-open: years.open;
        out property <[int]> pages: [YearPages.page-of(2026), YearPages.page-of(2028), YearPages.page-of(-5)];
        public function focus-months() { months.focus(); }
        public function focus-quarters() { quarters.focus(); }
        public function focus-years() { years.focus(); }
        months := MonthPicker {
            x: 10px; y: 10px; width: 200px;
            today: { year: 2026, month: 10, day: 27 };
            year <=> root.year;
            month <=> root.month;
            month-changed => { root.changes += 1; }
        }
        quarters := QuarterPicker {
            x: 10px; y: 50px; width: 200px;
            today: { year: 2026, month: 10, day: 27 };
            year <=> root.q-year;
            quarter <=> root.quarter;
        }
        years := YearPicker {
            x: 10px; y: 90px; width: 200px;
            today: { year: 2026, month: 10, day: 27 };
            year <=> root.pick-year;
            chosen <=> root.picked;
        }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    ely_slint::wire_dates!(h);
    show(&h);
    h
}

fn value_of(h: &Harness, label: &str) -> String {
    let e = i_slint_backend_testing::ElementHandle::find_by_accessible_label(h, label).next().unwrap_or_else(|| panic!("a field named {label}"));
    e.accessible_value().unwrap_or_default().to_string()
}

fn twelve_year_pages_start_on_a_multiple_of_twelve() {
    use slint::Model;
    let h = harness();
    assert_eq!(h.get_pages().iter().collect::<Vec<_>>(), [2016, 2028, -12]);
}

fn months_walk_turn_and_pick() {
    let h = harness();
    assert_eq!(value_of(&h, "Pick a month"), "October 2026");
    h.invoke_focus_months();
    tap(h.window(), Key::Return);
    assert!(h.get_month_open());
    // The cursor starts on October (index 9): down a row is beyond December, so it stops there.
    tap(h.window(), Key::DownArrow);
    tap(h.window(), Key::LeftArrow);
    tap(h.window(), Key::RightArrow);
    assert_eq!(value_of(&h, "2026"), "Dec");
    // A turn to a page without the choice puts the cursor on its first cell, as gpui re-seeds.
    tap(h.window(), Key::PageDown);
    tap(h.window(), Key::RightArrow);
    tap(h.window(), " ");
    assert!(!h.get_month_open());
    assert_eq!((h.get_year(), h.get_month()), (2027, 2));
    assert_eq!(h.get_changes(), 1);
    assert_eq!(value_of(&h, "Pick a month"), "February 2027");
}

fn quarters_pick_by_press() {
    let h = harness();
    assert_eq!(value_of(&h, "Pick a quarter"), "");
    h.invoke_focus_quarters();
    tap(h.window(), Key::DownArrow);
    assert!(h.get_quarter_open());
    click_label_in_popup(&h, "Q3", 10.0, 78.0);
    assert!(!h.get_quarter_open());
    assert_eq!((h.get_q_year(), h.get_quarter()), (2026, 3));
    assert_eq!(value_of(&h, "Pick a quarter"), "Q3 2026");
}

fn years_turn_by_twelve_and_keep_the_page() {
    let h = harness();
    h.invoke_focus_years();
    tap(h.window(), Key::Return);
    tap(h.window(), Key::PageUp);
    tap(h.window(), Key::Escape);
    assert!(!h.get_year_open());
    tap(h.window(), Key::Return);
    // The page stayed turned back: 2004 – 2015, cursor on its first cell.
    tap(h.window(), Key::RightArrow);
    tap(h.window(), Key::Return);
    assert!(h.get_picked());
    assert_eq!(h.get_pick_year(), 2005);
}

cases!(period: twelve_year_pages_start_on_a_multiple_of_twelve, months_walk_turn_and_pick, quarters_pick_by_press, years_turn_by_twelve_and_keep_the_page);
