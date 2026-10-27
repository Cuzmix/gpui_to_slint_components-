// Switch, after ely-core/src/forms/switch.rs.
#[macro_use]
mod common;
use common::*;
use slint::ComponentHandle;

slint::slint! {
    import { Switch } from "ui/index.slint";
    export component Harness inherits Window {
        width: 300px;
        height: 100px;
        in-out property <bool> wifi;
        in-out property <bool> enabled: true;
        in-out property <string> heard;
        out property <length> sw-width: sw.preferred-width;
        public function focus-sw() { sw.focus(); }
        sw := Switch {
            x: 10px;
            y: 10px;
            on <=> root.wifi;
            label: "Wi-Fi";
            enabled: root.enabled;
            toggled(on) => { root.heard += on ? "on " : "off "; }
        }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    show(&h);
    h
}

fn a_press_flips_it() {
    let h = harness();
    click(h.window(), 20.0, 19.0);
    advance(250);
    assert!(h.get_wifi());
    // The label toggles too.
    click(h.window(), 55.0, 19.0);
    assert!(!h.get_wifi());
    assert_eq!(h.get_heard(), "on off ");
}

fn keys_flip_it_and_disabled_does_not() {
    let h = harness();
    h.invoke_focus_sw();
    tap(h.window(), " ");
    assert!(h.get_wifi());
    h.set_enabled(false);
    click(h.window(), 20.0, 19.0);
    assert!(h.get_wifi());
}

fn track_then_label() {
    let h = harness();
    assert!(h.get_sw_width() > 32.0 + 8.0, "a 32 px track, 8 px, then the label");
}

cases!(switch: a_press_flips_it, keys_flip_it_and_disabled_does_not, track_then_label);
