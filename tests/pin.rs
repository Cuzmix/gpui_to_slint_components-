// PinInput, after ely-core/src/forms/pin.rs.
#[macro_use]
mod common;
use common::*;
use slint::ComponentHandle;
use slint::platform::Key;

slint::slint! {
    import { PinInput, TextFields, TextFit } from "ui/index.slint";
    export { TextFields, TextFit }
    export component Harness inherits Window {
        width: 500px;
        height: 200px;
        in-out property <int> attempt;
        in-out property <string> done;
        in-out property <int> completions;
        in-out property <bool> letters;
        out property <string> code: pin.code;
        out property <length> pin-width: pin.width;
        out property <length> pin-height: pin.height;
        public function focus-pin() { pin.focus(); }
        pin := PinInput {
            x: 10px;
            y: 10px;
            length: 4;
            letters: root.letters;
            attempt: root.attempt;
            label: "Code";
            completed(code) => { root.done = code; root.completions += 1; }
        }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    ely_slint::wire_fields!(h);
    show(&h);
    h
}

fn type_text(h: &Harness, text: &str) {
    for ch in text.chars() {
        tap(h.window(), ch.to_string());
    }
}

fn typing_fills_boxes_and_completes() {
    let h = harness();
    h.invoke_focus_pin();
    type_text(&h, "1a2");
    assert_eq!(h.get_code(), "12");
    assert_eq!(h.get_completions(), 0);
    type_text(&h, "34");
    assert_eq!(h.get_code(), "1234", "four boxes take four");
    assert_eq!(h.get_done(), "1234");
    assert_eq!(h.get_completions(), 1);
}

fn letters_come_out_upper_case() {
    let h = harness();
    h.set_letters(true);
    h.invoke_focus_pin();
    type_text(&h, "ab12");
    assert_eq!(h.get_done(), "AB12");
}

fn the_caret_stays_at_the_end() {
    let h = harness();
    h.invoke_focus_pin();
    type_text(&h, "12");
    tap(h.window(), Key::LeftArrow);
    advance(0);
    type_text(&h, "3");
    assert_eq!(h.get_code(), "123", "typing always lands at the end");
}

fn a_new_attempt_clears() {
    let h = harness();
    h.invoke_focus_pin();
    type_text(&h, "1234");
    h.set_attempt(1);
    advance(0);
    assert_eq!(h.get_code(), "");
}

fn boxes_are_a_large_control_and_a_quarter() {
    let h = harness();
    // Four 40 px boxes (lg 32 × 1.25), 8 px apart.
    assert_eq!(h.get_pin_height(), 40.0);
    assert_eq!(h.get_pin_width(), 4.0 * 40.0 + 3.0 * 8.0);
}

cases!(pin: typing_fills_boxes_and_completes, letters_come_out_upper_case, the_caret_stays_at_the_end, a_new_attempt_clears, boxes_are_a_large_control_and_a_quarter);
