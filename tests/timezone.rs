// TimezoneSelect, after ely-core/src/forms/date/zone.rs.
#[macro_use]
mod common;
use common::*;
use slint::ComponentHandle;
use slint::platform::Key;

slint::slint! {
    import { TimezoneSelect, TimeZones, Choice, Choices, Toggles, OptionFloat, OptionLayer, TextFields, TextFit } from "ui/index.slint";
    export { TimeZones, Choice, Choices, Toggles, TextFields, TextFit }
    export component Harness inherits Window {
        width: 400px;
        height: 400px;
        in-out property <string> zone: "Asia/Tokyo";
        in-out property <int> changes;
        out property <string> text: pick.text;
        out property <int> rows: OptionFloat.owner == "" ? 0 : OptionFloat.rows.length;
        out property <string> first-note: OptionFloat.rows.length > 0 ? OptionFloat.rows[0].note : "";
        out property <string> first-label: OptionFloat.rows.length > 0 ? OptionFloat.rows[0].label : "";
        public function focus-pick() { pick.focus(); }
        pick := TimezoneSelect {
            x: 10px; y: 10px; width: 260px;
            zone <=> root.zone;
            zone-changed => { root.changes += 1; }
        }
        OptionLayer { }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    ely_slint::wire_fields!(h);
    ely_slint::wire_options!(h);
    ely_slint::wire_toggles!(h);
    ely_slint::wire_zones!(h);
    show(&h);
    h
}

fn it_lists_zones_with_offsets_and_picks_by_name() {
    let h = harness();
    assert_eq!(h.get_text(), "Asia/Tokyo");
    h.invoke_focus_pick();
    advance(0);
    assert!(h.get_rows() > 100, "the system's zones: {}", h.get_rows());
    tap(h.window(), Key::End);
    for _ in 0.."Asia/Tokyo".len() {
        tap(h.window(), Key::Backspace);
    }
    for ch in "Kolkata".chars() {
        tap(h.window(), ch.to_string());
    }
    advance(0);
    assert_eq!(h.get_first_label(), "Asia/Kolkata");
    assert_eq!(h.get_first_note(), "UTC+05:30");
    tap(h.window(), Key::Return);
    advance(0);
    assert_eq!(h.get_zone(), "Asia/Kolkata");
    assert_eq!(h.get_changes(), 1);
}

cases!(timezone: it_lists_zones_with_offsets_and_picks_by_name);
