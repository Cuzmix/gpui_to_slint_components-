// Combobox, after ely-core/src/forms/combobox.rs.
#[macro_use]
mod common;
use common::*;
use slint::ComponentHandle;
use slint::platform::Key;

slint::slint! {
    import { Combobox, Choice, Choices, Toggles, OptionFloat, OptionLayer, TextFields, TextFit } from "ui/index.slint";
    export { Choice, Choices, Toggles, TextFields, TextFit }
    export component Harness inherits Window {
        width: 400px;
        height: 400px;
        in-out property <string> city: "lis";
        in-out property <bool> free;
        in-out property <int> changes;
        out property <bool> open: combo.open;
        out property <int> highlighted: combo.highlighted;
        out property <string> text: combo.text;
        out property <int> rows: OptionFloat.owner == "" ? 0 : OptionFloat.rows.length;
        public function focus-combo() { combo.focus(); }
        combo := Combobox {
            x: 10px;
            y: 10px;
            width: 200px;
            free: root.free;
            choices: [
                { value: "ber", label: "Berlin" },
                { value: "lis", label: "Lisbon" },
                { value: "lon", label: "London", disabled: true },
                { value: "oslo", label: "Oslo", note: "NO" },
            ];
            selected <=> root.city;
            selection-changed => { root.changes += 1; }
        }
        other := FocusScope { x: 300px; y: 300px; width: 20px; height: 20px; }
        public function focus-other() { other.focus(); }
        OptionLayer { }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    ely_slint::wire_fields!(h);
    ely_slint::wire_options!(h);
    ely_slint::wire_toggles!(h);
    show(&h);
    h
}

fn typing(h: &Harness, text: &str) {
    for ch in text.chars() {
        tap(h.window(), ch.to_string());
    }
    advance(0);
}

fn shows_the_label_and_every_row_on_focus() {
    let h = harness();
    assert_eq!(h.get_text(), "Lisbon");
    h.invoke_focus_combo();
    advance(0);
    assert!(h.get_open());
    assert_eq!(h.get_rows(), 4, "the chosen label shows every row");
    assert_eq!(h.get_highlighted(), 1, "the choice");
}

fn typing_filters_and_enter_picks() {
    let h = harness();
    h.invoke_focus_combo();
    tap(h.window(), Key::End);
    for _ in 0..6 {
        tap(h.window(), Key::Backspace);
    }
    typing(&h, "o");
    // Berlin? no; Lisbon, London, Oslo hold "o".
    assert_eq!(h.get_rows(), 3);
    tap(h.window(), Key::DownArrow);
    assert_eq!(h.get_highlighted(), 2, "London is disabled");
    tap(h.window(), Key::Return);
    advance(0);
    assert_eq!(h.get_city(), "oslo");
    assert_eq!(h.get_text(), "Oslo");
    assert!(!h.get_open(), "a pick hides the list");
    tap(h.window(), Key::DownArrow);
    advance(0);
    assert!(h.get_open(), "Down shows it again");
}

fn escape_hides_until_typing() {
    let h = harness();
    h.invoke_focus_combo();
    advance(0);
    tap(h.window(), Key::Escape);
    advance(0);
    assert!(!h.get_open());
    typing(&h, "x");
    assert!(!h.get_open(), "no row holds the text");
}

fn leaving_puts_the_label_back_unless_free() {
    let h = harness();
    h.invoke_focus_combo();
    typing(&h, "zz");
    h.invoke_focus_other();
    advance(0);
    assert_eq!(h.get_text(), "Lisbon");
    h.set_free(true);
    h.invoke_focus_combo();
    tap(h.window(), Key::End);
    typing(&h, "zz");
    h.invoke_focus_other();
    advance(0);
    assert_eq!(h.get_text(), "Lisbonzz", "free keeps any text");
}

fn a_new_choice_shows_while_unfocused() {
    let h = harness();
    h.set_city("ber".into());
    advance(0);
    assert_eq!(h.get_text(), "Berlin");
}

cases!(combobox: shows_the_label_and_every_row_on_focus, typing_filters_and_enter_picks, escape_hides_until_typing, leaving_puts_the_label_back_unless_free, a_new_choice_shows_while_unfocused);
