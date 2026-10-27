// CronEditor, after ely-core/src/forms/date/cron.rs and rule.rs.
#[macro_use]
mod common;
use common::*;
use slint::ComponentHandle;

slint::slint! {
    import { CronEditor, Crons, CronReading, CivilDate, CivilTime, TextFields, TextFit } from "ui/index.slint";
    export { Crons, CronReading, CivilDate, CivilTime, TextFields, TextFit }
    export component Harness inherits Window {
        width: 600px;
        height: 300px;
        in-out property <string> rule;
        in-out property <int> changes;
        out property <CronReading> reading: editor.reading;
        public function focus-editor() { editor.focus(); }
        editor := CronEditor {
            x: 10px; y: 10px; width: 560px;
            now-date: { year: 2026, month: 9, day: 25 };
            now-time: { hour: 12, minute: 0 };
            rule <=> root.rule;
            rule-changed => { root.changes += 1; }
        }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    ely_slint::wire_fields!(h);
    ely_slint::wire_crons!(h);
    show(&h);
    h
}

fn blank_errors_and_rules_read_back() {
    let h = harness();
    assert_eq!(h.get_reading().state, 0);
    h.set_rule("61 * * * *".into());
    let reading = h.get_reading();
    assert_eq!((reading.state, reading.words.as_str()), (1, "minute 61 is outside 0 to 59"));
    h.set_rule("30 9 * * 1-5".into());
    let reading = h.get_reading();
    assert_eq!(reading.state, 2);
    assert_eq!(reading.words.as_str(), "At 09:30, Monday through Friday");
    assert_eq!(reading.runs.as_str(), "Mon Sep 28, 09:30 \u{00b7} Tue Sep 29, 09:30 \u{00b7} Wed Sep 30, 09:30");
    h.set_rule("0 0 30 2 *".into());
    assert!(!h.get_reading().any);
}

fn presets_fill_the_field() {
    let h = harness();
    click_label(&h, "Weekdays at 9:00");
    assert_eq!(h.get_rule(), "0 9 * * 1-5");
    assert_eq!(h.get_changes(), 1);
    assert_eq!(h.get_reading().words.as_str(), "At 09:00, Monday through Friday");
    h.invoke_focus_editor();
    tap(h.window(), slint::platform::Key::End);
    tap(h.window(), slint::platform::Key::Backspace);
    tap(h.window(), "3");
    assert_eq!(h.get_rule(), "0 9 * * 1-3");
    assert_eq!(h.get_reading().words.as_str(), "At 09:00, Monday through Wednesday");
}

cases!(cron: blank_errors_and_rules_read_back, presets_fill_the_field);
