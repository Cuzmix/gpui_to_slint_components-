#[macro_use]
mod common;
use common::*;
use slint::ComponentHandle;

slint::slint! {
    import { Button, FocusTrap, FocusSteps } from "ui/index.slint";
    export component Harness inherits Window {
        width: 400px;
        height: 300px;
        in-out property <bool> trapping: true;
        in-out property <bool> with-stops: true;
        out property <string> focused: before.has-focus ? "before" : one.has-focus ? "one" : two.has-focus ? "two" : three.has-focus ? "three" : after.has-focus ? "after" : "";
        VerticalLayout {
            alignment: start;
            before := Button { text: "Before"; }
            FocusTrap {
                trapping: trapping;
                HorizontalLayout {
                    one := Button { text: "One"; enabled: with-stops; }
                    two := Button { text: "Two"; enabled: with-stops; }
                    three := Button { text: "Three"; enabled: with-stops; }
                }
            }
            after := Button { text: "After"; }
        }
    }
    export { FocusSteps }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    let weak = h.as_weak();
    h.global::<FocusSteps>().on_step(move |forward| {
        if let Some(h) = weak.upgrade() {
            ely_slint::focus::step(h.window(), forward);
        }
    });
    show(&h);
    h
}

// Each Tab goes through one event; the trap clears its heading 1 ms later.
fn tab_(h: &Harness) {
    tab(h.window());
    advance(2);
}

fn shift_tab_(h: &Harness) {
    shift_tab(h.window());
    advance(2);
}

fn tab_enters_then_circles_inside() {
    let h = harness();
    tab_(&h);
    assert_eq!(h.get_focused(), "before");
    for want in ["one", "two", "three", "one", "two", "three", "one"] {
        tab_(&h);
        assert_eq!(h.get_focused(), want);
    }
}

fn shift_tab_circles_backwards() {
    let h = harness();
    tab_(&h);
    tab_(&h);
    assert_eq!(h.get_focused(), "one");
    for want in ["three", "two", "one", "three"] {
        shift_tab_(&h);
        assert_eq!(h.get_focused(), want);
    }
}

fn shift_tab_from_after_enters_at_the_last_stop() {
    let h = harness();
    for _ in 0..2 {
        shift_tab_(&h);
    }
    // Shift-Tab from nothing wraps to the window's last stop, After, then into the trap.
    assert_eq!(h.get_focused(), "three");
}

fn untrapped_lets_tab_leave() {
    let h = harness();
    h.set_trapping(false);
    let mut seen = vec![];
    for _ in 0..5 {
        tab_(&h);
        seen.push(h.get_focused().to_string());
    }
    assert_eq!(seen, ["before", "one", "two", "three", "after"]);
}

// GPUI steps inside a trap only while focus is in it; a trap with no stop is passed over.
fn trap_without_stops_is_passed_over() {
    let h = harness();
    h.set_with_stops(false);
    tab_(&h);
    tab_(&h);
    assert_eq!(h.get_focused(), "after");
    shift_tab_(&h);
    assert_eq!(h.get_focused(), "before");
}

cases!(focus_trap: tab_enters_then_circles_inside, shift_tab_circles_backwards, shift_tab_from_after_enters_at_the_last_stop, untrapped_lets_tab_leave, trap_without_stops_is_passed_over);
