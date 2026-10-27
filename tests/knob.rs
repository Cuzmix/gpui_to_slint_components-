// Knob, after ely-core/src/forms/knob.rs.
#[macro_use]
mod common;
use common::*;
use slint::ComponentHandle;
use slint::platform::Key;

slint::slint! {
    import { Knob } from "ui/index.slint";
    export component Harness inherits Window {
        width: 200px;
        height: 400px;
        in-out property <float> gain: 0;
        in-out property <int> changes;
        in-out property <bool> enabled: true;
        out property <bool> focused: dial.has-focus;
        out property <length> size: dial.width;
        dial := Knob {
            x: 10px;
            y: 150px;
            label: "Gain";
            minimum: -24;
            maximum: 24;
            step: 0.5;
            enabled: root.enabled;
            value <=> root.gain;
            value-changed => { root.changes += 1; }
        }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    show(&h);
    h
}

fn it_is_a_48_px_dial() {
    let h = harness();
    assert_eq!(h.get_size(), 48.0);
}

fn dragging_up_turns_it_up() {
    let h = harness();
    mouse_down(h.window(), 34.0, 174.0);
    assert!(h.get_focused(), "a press focuses");
    // The first move only anchors the drag.
    mouse_move(h.window(), 34.0, 170.0);
    assert_eq!(h.get_gain(), 0.0);
    // Four diameters (192 px) span the range: 48 px is a quarter, 12 units.
    mouse_move(h.window(), 34.0, 122.0);
    assert_eq!(h.get_gain(), 12.0);
    mouse_move(h.window(), 34.0, 0.0);
    assert_eq!(h.get_gain(), 24.0);
    mouse_move(h.window(), 34.0, 218.0);
    assert_eq!(h.get_gain(), -12.0);
    mouse_up(h.window(), 34.0, 218.0);
    let changes = h.get_changes();
    mouse_move(h.window(), 34.0, 100.0);
    assert_eq!(h.get_changes(), changes, "the drag ended");
}

fn keys_step() {
    let h = harness();
    click(h.window(), 34.0, 174.0);
    tap(h.window(), Key::UpArrow);
    assert_eq!(h.get_gain(), 0.5);
    tap(h.window(), Key::PageDown);
    assert_eq!(h.get_gain(), -4.5);
    tap(h.window(), Key::End);
    assert_eq!(h.get_gain(), 24.0);
}

fn disabled_takes_no_turn() {
    let h = harness();
    h.set_enabled(false);
    mouse_down(h.window(), 34.0, 174.0);
    mouse_move(h.window(), 34.0, 170.0);
    mouse_move(h.window(), 34.0, 100.0);
    mouse_up(h.window(), 34.0, 100.0);
    assert_eq!(h.get_gain(), 0.0);
    assert!(!h.get_focused());
}

cases!(knob: it_is_a_48_px_dial, dragging_up_turns_it_up, keys_step, disabled_takes_no_turn);
