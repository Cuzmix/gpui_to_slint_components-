#[macro_use]
mod common;
use common::*;

slint::slint! {
    import { TooltipTrigger, TooltipController, HelpTooltip, Button } from "ui/index.slint";
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
        in-out property <int> go-clicks;
        out property <bool> button-trigger-shown: with-button.shown;
        out property <bool> rich-shown: rich-tip.shown;
        TouchArea {
            x: 300px;
            y: 200px;
            width: 50px;
            height: 50px;
            rich-tip := TooltipController {
                rich: true;
                Text { text: "Rich content"; }
            }
            pointer-event(event) => { rich-tip.pointer-event(event, self.mouse-x, self.mouse-y); }
        }
        with-button := TooltipTrigger {
            x: 0;
            y: 200px;
            width: 100px;
            height: 40px;
            text: "Runs it";
            Button { text: "Go"; clicked => { go-clicks += 1; } }
        }
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

// GPUI hides on any mouse down inside the trigger, also one its child button handles.
fn trigger_hides_on_a_press_its_child_handles() {
    let h = harness();
    mouse_move(h.window(), 10.0, 210.0);
    advance(400);
    assert!(h.get_button_trigger_shown());
    click(h.window(), 10.0, 210.0);
    assert_eq!(h.get_go_clicks(), 1);
    assert!(!h.get_button_trigger_shown());
}

// A rich tip has no text of its own and still shows.
fn rich_tip_shows_its_children() {
    let h = harness();
    mouse_move(h.window(), 310.0, 210.0);
    advance(500);
    assert!(h.get_rich_shown());
    let found: Vec<_> = i_slint_backend_testing::ElementHandle::find_by_accessible_label(&h, "Rich content").collect();
    assert!(!found.is_empty());
}

cases!(tooltip: trigger_waits_400ms_from_entering, trigger_hides_on_press_and_on_leave, help_shows_after_500ms_and_hides_on_leave, trigger_hides_on_a_press_its_child_handles, rich_tip_shows_its_children);
