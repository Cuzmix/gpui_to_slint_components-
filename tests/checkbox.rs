// Checkbox, CheckboxGroup, after ely-core/src/forms/check.rs.
#[macro_use]
mod common;
use common::*;
use slint::{ComponentHandle, Model};

slint::slint! {
    import { Checkbox, CheckboxGroup, CheckState, Choice, Choices, Toggles } from "ui/index.slint";
    export { Choice, Choices, Toggles, CheckState }
    export component Harness inherits Window {
        width: 400px;
        height: 300px;
        in-out property <CheckState> terms: CheckState.mixed;
        in-out property <bool> terms-enabled: true;
        in-out property <string> heard;
        in-out property <[string]> days: ["tue"];
        in-out property <int> group-changes;
        out property <bool> terms-focused: box.has-focus;
        public function focus-box() { box.focus(); }
        box := Checkbox {
            x: 10px;
            y: 10px;
            state <=> root.terms;
            label: "I accept the terms";
            enabled: root.terms-enabled;
            toggled(on) => { root.heard += on ? "on " : "off "; }
        }
        CheckboxGroup {
            x: 10px;
            y: 100px;
            width: 200px;
            choices: [
                { value: "mon", label: "Monday" },
                { value: "tue", label: "Tuesday" },
                { value: "wed", label: "Wednesday", disabled: true },
            ];
            selected <=> root.days;
            selection-changed => { root.group-changes += 1; }
        }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    ely_slint::wire_toggles!(h);
    ely_slint::wire_options!(h);
    show(&h);
    h
}

fn days(h: &Harness) -> Vec<String> {
    h.get_days().iter().map(|d| d.to_string()).collect()
}

fn mixed_turns_on_then_off() {
    let h = harness();
    // The label is part of the row.
    click(h.window(), 80.0, 20.0);
    assert_eq!(h.get_terms(), CheckState::On);
    click(h.window(), 18.0, 20.0);
    assert_eq!(h.get_terms(), CheckState::Off);
    assert_eq!(h.get_heard(), "on off ");
    assert!(!h.get_terms_focused(), "a press takes no focus");
}

fn space_and_enter_toggle_when_focused() {
    let h = harness();
    h.invoke_focus_box();
    tap(h.window(), " ");
    assert_eq!(h.get_terms(), CheckState::On);
    tap(h.window(), slint::platform::Key::Return);
    assert_eq!(h.get_terms(), CheckState::Off);
}

fn disabled_takes_no_press() {
    let h = harness();
    h.set_terms_enabled(false);
    click(h.window(), 18.0, 20.0);
    assert_eq!(h.get_terms(), CheckState::Mixed);
    assert_eq!(h.get_heard(), "");
}

fn a_group_keeps_choice_order() {
    let h = harness();
    // Monday is the first row, 8 px above Tuesday.
    click(h.window(), 18.0, 110.0);
    assert_eq!(days(&h), ["mon", "tue"]);
    assert_eq!(h.get_group_changes(), 1);
    // The owner's new selection still shows: the boxes stay bound to it.
    h.set_days(slint::ModelRc::new(slint::VecModel::from(vec![slint::SharedString::from("tue")])));
    click(h.window(), 18.0, 110.0);
    assert_eq!(days(&h), ["mon", "tue"], "Monday was off again, so the press turns it on");
}

fn the_motion_settles() {
    let h = harness();
    click(h.window(), 18.0, 20.0);
    advance(200);
    assert_eq!(h.get_terms(), CheckState::On);
}

cases!(checkbox: mixed_turns_on_then_off, space_and_enter_toggle_when_focused, disabled_takes_no_press, a_group_keeps_choice_order, the_motion_settles);
