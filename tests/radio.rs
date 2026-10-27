// Radio, RadioGroup, after ely-core/src/forms/radio.rs.
#[macro_use]
mod common;
use common::*;
use slint::ComponentHandle;
use slint::platform::Key;

slint::slint! {
    import { Radio, RadioGroup, Choice, Choices, FocusRing } from "ui/index.slint";
    export { Choice, Choices }
    export component Harness inherits Window {
        width: 400px;
        height: 300px;
        in-out property <bool> monthly;
        in-out property <int> selects;
        in-out property <string> billing: "b";
        in-out property <int> changes;
        out property <bool> lone-focused: lone.has-focus;
        lone := Radio {
            x: 10px;
            y: 10px;
            on <=> root.monthly;
            label: "Monthly";
            selected => { root.selects += 1; }
        }
        // Takes Tab first.
        first := FocusScope { x: 300px; y: 10px; width: 10px; height: 10px; }
        RadioGroup {
            x: 10px;
            y: 100px;
            width: 200px;
            choices: [
                { value: "a", label: "Alpha" },
                { value: "b", label: "Beta" },
                { value: "c", label: "Gamma", disabled: true },
                { value: "d", label: "Delta" },
            ];
            selected <=> root.billing;
            selection-changed => { root.changes += 1; }
        }
        public function focus-first() { first.focus(); }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    ely_slint::wire_options!(h);
    show(&h);
    h
}

fn a_radio_picks_once() {
    let h = harness();
    click(h.window(), 18.0, 20.0);
    assert!(h.get_monthly());
    click(h.window(), 18.0, 20.0);
    assert_eq!(h.get_selects(), 1, "pressing an on radio does nothing");
    assert!(!h.get_lone_focused());
}

fn a_group_is_one_tab_stop_at_the_choice() {
    let h = harness();
    h.invoke_focus_first();
    tab(h.window());
    // Beta has the stop: an arrow from it moves to Delta, past disabled Gamma.
    tap(h.window(), Key::DownArrow);
    assert_eq!(h.get_billing(), "d");
    tap(h.window(), Key::RightArrow);
    assert_eq!(h.get_billing(), "a", "wraps");
    tap(h.window(), Key::UpArrow);
    assert_eq!(h.get_billing(), "d");
    assert_eq!(h.get_changes(), 3);
}

fn a_press_picks_and_the_owner_rules() {
    let h = harness();
    // Alpha is the first row.
    click(h.window(), 18.0, 110.0);
    assert_eq!(h.get_billing(), "a");
    h.set_billing("b".into());
    click(h.window(), 18.0, 110.0);
    assert_eq!(h.get_billing(), "a", "the group still follows the owner");
}

cases!(radio: a_radio_picks_once, a_group_is_one_tab_stop_at_the_choice, a_press_picks_and_the_owner_rules);
