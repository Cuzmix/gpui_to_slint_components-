// DatePicker and DateRangePicker, after ely-core/src/forms/date/picker.rs.
#[macro_use]
mod common;
use common::*;
use slint::ComponentHandle;
use slint::platform::Key;

slint::slint! {
    import { DatePicker, DateRangePicker, CivilDate, Dates } from "ui/index.slint";
    export { CivilDate, Dates }
    export component Harness inherits Window {
        width: 400px;
        height: 500px;
        in-out property <CivilDate> due: { year: 2026, month: 10, day: 23 };
        in-out property <int> changes;
        in-out property <CivilDate> first;
        in-out property <CivilDate> last;
        in-out property <int> spans;
        out property <bool> open: pick.open;
        out property <bool> focused: pick.has-focus;
        out property <bool> range-open: trip.open;
        public function focus-pick() { pick.focus(); }
        public function focus-trip() { trip.focus(); }
        pick := DatePicker {
            x: 10px;
            y: 10px;
            width: 200px;
            today: { year: 2026, month: 10, day: 20 };
            value <=> root.due;
            date-changed => { root.changes += 1; }
        }
        trip := DateRangePicker {
            x: 10px;
            y: 60px;
            width: 200px;
            today: { year: 2026, month: 10, day: 20 };
            first <=> root.first;
            last <=> root.last;
            range-changed => { root.spans += 1; }
        }
    }
}

// What the field reads to assistive technology.
fn value_of(h: &Harness, label: &str) -> String {
    let e = i_slint_backend_testing::ElementHandle::find_by_accessible_label(h, label).next().unwrap_or_else(|| panic!("a field named {label}"));
    e.accessible_value().unwrap_or_default().to_string()
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    ely_slint::wire_dates!(h);
    show(&h);
    h
}

fn ymd(d: CivilDate) -> (i32, i32, i32) {
    (d.year, d.month, d.day)
}

fn keys_open_walk_and_pick() {
    let h = harness();
    assert_eq!(value_of(&h, "Pick a date"), "Oct 23, 2026");
    h.invoke_focus_pick();
    tap(h.window(), Key::DownArrow);
    assert!(h.get_open());
    assert!(!h.get_focused(), "focus moved into the calendar");
    tap(h.window(), Key::RightArrow);
    tap(h.window(), Key::Return);
    assert!(!h.get_open());
    assert!(h.get_focused(), "focus came back");
    assert_eq!(ymd(h.get_due()), (2026, 10, 24));
    assert_eq!(h.get_changes(), 1);
    tap(h.window(), " ");
    assert!(h.get_open());
    tap(h.window(), Key::Escape);
    assert!(!h.get_open());
    assert!(h.get_focused());
    assert_eq!(h.get_changes(), 1);
}

fn a_press_toggles_and_a_day_press_picks() {
    let h = harness();
    click(h.window(), 50.0, 24.0);
    assert!(h.get_open());
    click_label_in_popup(&h, "Oct 9, 2026", 10.0, 38.0);
    assert!(!h.get_open());
    assert_eq!(ymd(h.get_due()), (2026, 10, 9));
    click(h.window(), 50.0, 24.0);
    assert!(h.get_open());
    click(h.window(), 380.0, 480.0);
    assert!(!h.get_open(), "a press outside closes");
    assert!(h.get_focused());
}

fn a_range_takes_two_picks_in_either_order() {
    let h = harness();
    h.invoke_focus_trip();
    tap(h.window(), Key::Return);
    assert!(h.get_range_open());
    click_label_in_popup(&h, "Oct 17, 2026", 10.0, 88.0);
    assert!(h.get_range_open(), "one end is not a span");
    click_label_in_popup(&h, "Oct 3, 2026", 10.0, 88.0);
    assert!(!h.get_range_open());
    assert_eq!((ymd(h.get_first()), ymd(h.get_last())), ((2026, 10, 3), (2026, 10, 17)));
    assert_eq!(h.get_spans(), 1);
    assert_eq!(value_of(&h, "Pick dates"), "Oct 3 \u{2013} 17, 2026");
}

cases!(date_picker: keys_open_walk_and_pick, a_press_toggles_and_a_day_press_picks, a_range_takes_two_picks_in_either_order);
