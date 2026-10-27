// WeekPicker, after ely-core/src/forms/date/week.rs.
#[macro_use]
mod common;
use common::*;
use slint::ComponentHandle;
use slint::platform::Key;

slint::slint! {
    import { WeekPicker, CivilDate, IsoWeek, Dates } from "ui/index.slint";
    export { CivilDate, IsoWeek, Dates }
    export component Harness inherits Window {
        width: 400px;
        height: 500px;
        in-out property <int> year: 2026;
        in-out property <int> week: 43;
        in-out property <int> changes;
        out property <bool> open: sprint.open;
        public function focus-sprint() { sprint.focus(); }
        sprint := WeekPicker {
            x: 10px; y: 10px; width: 200px;
            today: { year: 2026, month: 10, day: 27 };
            year <=> root.year;
            week <=> root.week;
            week-changed => { root.changes += 1; }
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

fn selected(h: &Harness, label: &str) -> bool {
    let e = i_slint_backend_testing::ElementHandle::find_by_accessible_label(h, label).next().unwrap_or_else(|| panic!("a day named {label}"));
    e.accessible_item_selected().unwrap_or(false)
}

fn it_shows_and_marks_the_week() {
    let h = harness();
    assert_eq!(value_of(&h, "Pick a week"), "Week 43, 2026");
    h.invoke_focus_sprint();
    tap(h.window(), Key::Return);
    assert!(h.get_open());
    // Week 43 of 2026 runs Monday Oct 19 to Sunday Oct 25: its ends are chosen.
    assert!(selected(&h, "Oct 19, 2026"));
    assert!(selected(&h, "Oct 25, 2026"));
    assert!(!selected(&h, "Oct 22, 2026"), "the middle is the band");
}

fn a_day_picks_its_iso_week() {
    let h = harness();
    h.invoke_focus_sprint();
    tap(h.window(), Key::Return);
    // The calendar opens on the span start, Oct 19, with the cursor there.
    tap(h.window(), Key::PageDown);
    tap(h.window(), Key::PageDown);
    tap(h.window(), Key::PageDown);
    // Jan 19, 2027 is in ISO week 3 of 2027.
    tap(h.window(), Key::Return);
    assert!(!h.get_open());
    assert_eq!((h.get_year(), h.get_week()), (2027, 3));
    assert_eq!(h.get_changes(), 1);
}

cases!(week: it_shows_and_marks_the_week, a_day_picks_its_iso_week);
