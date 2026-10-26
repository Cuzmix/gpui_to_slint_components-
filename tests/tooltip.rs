#[macro_use]
mod common;
use common::*;

slint::slint! {
    import { TooltipTrigger, HelpTooltip } from "ui/index.slint";
    export component Harness inherits Window {
        width: 400px;
        height: 300px;
        out property <bool> trigger-shown: trig.shown;
        out property <bool> help-shown: help.shown;
        trig := TooltipTrigger {
            x: 0;
            y: 0;
            width: 100px;
            height: 50px;
            text: "42 visits";
            Rectangle { }
        }
        help := HelpTooltip { x: 200px; y: 0; text: "Requests per minute"; }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    show(&h);
    h
}

fn trigger_waits_400ms_from_entering() {
    let h = harness();
    mouse_move(h.window(), 10.0, 10.0);
    advance(399);
    assert!(!h.get_trigger_shown());
    advance(1);
    assert!(h.get_trigger_shown());
}

fn trigger_hides_on_press_and_on_leave() {
    let h = harness();
    mouse_move(h.window(), 10.0, 10.0);
    advance(400);
    mouse_down(h.window(), 10.0, 10.0);
    assert!(!h.get_trigger_shown());
    mouse_up(h.window(), 10.0, 10.0);
    mouse_move(h.window(), 300.0, 200.0);
    mouse_move(h.window(), 10.0, 10.0);
    advance(400);
    assert!(h.get_trigger_shown());
    mouse_move(h.window(), 300.0, 200.0);
    assert!(!h.get_trigger_shown());
}

// HelpTooltip uses gpui's own tooltip: 500 ms from the first move over the mark.
fn help_shows_after_500ms_and_hides_on_leave() {
    let h = harness();
    mouse_move(h.window(), 204.0, 4.0);
    advance(499);
    assert!(!h.get_help_shown());
    advance(1);
    assert!(h.get_help_shown());
    mouse_move(h.window(), 300.0, 200.0);
    assert!(!h.get_help_shown());
}

cases!(tooltip: trigger_waits_400ms_from_entering, trigger_hides_on_press_and_on_leave, help_shows_after_500ms_and_hides_on_leave);
