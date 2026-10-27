// TimePicker, DateTimePicker and DurationPicker, after ely-core/src/forms/date/time.rs.
#[macro_use]
mod common;
use common::*;
use slint::ComponentHandle;
use slint::platform::Key;

slint::slint! {
    import { TimePicker, DateTimePicker, DurationPicker, TimeMath, CivilDate, CivilTime, IsoWeek, Dates, Numbers, ParsedNumber, TextFields, TextFit } from "ui/index.slint";
    export { CivilDate, IsoWeek, Dates, Numbers, ParsedNumber, TextFields, TextFit }
    export component Harness inherits Window {
        width: 600px;
        height: 500px;
        in-out property <CivilTime> alarm: { hour: 7, minute: 30 };
        in-out property <bool> alarm-set: true;
        in-out property <int> changes;
        in-out property <CivilDate> day;
        in-out property <CivilTime> at;
        in-out property <int> moments;
        in-out property <int> total: 3 * 3600 + 25 * 60 + 9;
        in-out property <int> durations;
        out property <bool> open: alarm-pick.open;
        out property <bool> moment-open: meeting.open;
        out property <string> wrapped: TimeMath.text(TimeMath.nudged({ hour: 23, minute: 55 }, 10));
        out property <string> back: TimeMath.text(TimeMath.nudged({ hour: 0, minute: 0 }, -5));
        out property <length> duration-width: span.preferred-width;
        public function focus-alarm() { alarm-pick.focus(); }
        public function focus-meeting() { meeting.focus(); }
        alarm-pick := TimePicker {
            x: 10px; y: 10px; width: 200px;
            step: 15;
            time <=> root.alarm;
            chosen <=> root.alarm-set;
            time-changed => { root.changes += 1; }
        }
        meeting := DateTimePicker {
            x: 250px; y: 10px; width: 200px;
            step: 30;
            today: { year: 2026, month: 10, day: 27 };
            date <=> root.day;
            time <=> root.at;
            moment-changed => { root.moments += 1; }
        }
        span := DurationPicker {
            x: 10px; y: 440px;
            seconds: true;
            value <=> root.total;
            duration-changed => { root.durations += 1; }
        }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    ely_slint::wire_dates!(h);
    ely_slint::wire_numbers!(h);
    ely_slint::wire_fields!(h);
    show(&h);
    h
}

// The first value an element named `label` reports (a number field's frame and editor share it).
fn value_of(h: &Harness, label: &str) -> String {
    i_slint_backend_testing::ElementHandle::find_by_accessible_label(h, label)
        .filter_map(|e| e.accessible_value())
        .find(|value| !value.is_empty())
        .map(|value| value.to_string())
        .unwrap_or_default()
}

fn hm(t: CivilTime) -> (i32, i32) {
    (t.hour, t.minute)
}

fn times_wrap() {
    let h = harness();
    assert_eq!(h.get_wrapped(), "00:05");
    assert_eq!(h.get_back(), "23:55");
}

fn keys_step_the_time_and_close() {
    let h = harness();
    assert_eq!(value_of(&h, "Pick a time"), "07:30");
    h.invoke_focus_alarm();
    tap(h.window(), Key::DownArrow);
    assert!(h.get_open());
    assert_eq!(value_of(&h, "Time"), "07:30");
    tap(h.window(), Key::DownArrow);
    assert_eq!(hm(h.get_alarm()), (7, 45));
    tap(h.window(), Key::UpArrow);
    tap(h.window(), Key::UpArrow);
    assert_eq!(hm(h.get_alarm()), (7, 15));
    assert!(h.get_open(), "a step keeps it open");
    tap(h.window(), Key::Return);
    assert!(!h.get_open());
    assert_eq!(h.get_changes(), 3);
}

fn a_row_press_sets_that_part() {
    let h = harness();
    click(h.window(), 50.0, 24.0);
    assert!(h.get_open());
    // Rows are 24 px from 4 px in, scrolled so the chosen row sits two down: hour 7 at row 2.
    click_label_in_popup(&h, "09", 10.0, 38.0);
    assert_eq!(hm(h.get_alarm()), (9, 30));
    click_label_in_popup(&h, "45", 10.0, 38.0);
    assert_eq!(hm(h.get_alarm()), (9, 45));
    assert!(h.get_open());
}

fn a_moment_takes_a_day_and_a_time() {
    let h = harness();
    assert_eq!(value_of(&h, "Pick a date and time"), "");
    h.invoke_focus_meeting();
    tap(h.window(), Key::Return);
    assert!(h.get_moment_open());
    // A time first: the day is today.
    click_label_in_popup(&h, "10", 250.0, 38.0);
    assert_eq!((h.get_day().day, hm(h.get_at())), (27, (10, 0)));
    // The calendar has focus: a day keeps the time.
    tap(h.window(), Key::RightArrow);
    tap(h.window(), Key::Return);
    assert_eq!((h.get_day().day, hm(h.get_at())), (28, (10, 0)));
    assert!(h.get_moment_open(), "a pick keeps it open");
    assert_eq!(value_of(&h, "Pick a date and time"), "Oct 28, 2026 10:00");
    tap(h.window(), Key::Escape);
    assert!(!h.get_moment_open());
    assert_eq!(h.get_moments(), 2);
}

fn durations_split_and_join() {
    let h = harness();
    assert_eq!(value_of(&h, "Hours"), "3");
    assert_eq!(value_of(&h, "Seconds"), "9");
    assert!(h.get_duration_width() >= 3.0 * 112.0 + 16.0);
    click_label(&h, "Minutes");
    tap(h.window(), Key::UpArrow);
    assert_eq!(h.get_total(), 3 * 3600 + 26 * 60 + 9);
    assert_eq!(h.get_durations(), 1);
    h.set_total(90);
    // Change handlers run between frames: one splits the value, the next shows each part.
    advance(0);
    advance(0);
    assert_eq!(value_of(&h, "Hours"), "0");
    assert_eq!(value_of(&h, "Seconds"), "30");
}

cases!(time: times_wrap, keys_step_the_time_and_close, a_row_press_sets_that_part, a_moment_takes_a_day_and_a_time, durations_split_and_join);
