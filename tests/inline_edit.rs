// InlineEdit, after ely-core/src/forms/inline.rs.
#[macro_use]
mod common;
use common::*;
use slint::ComponentHandle;
use slint::platform::Key;

slint::slint! {
    import { InlineEdit, TextFields, TextFit } from "ui/index.slint";
    export { TextFields, TextFit }
    export component Harness inherits Window {
        width: 500px;
        height: 200px;
        in-out property <string> value: "Draft";
        in-out property <int> commits;
        out property <bool> editing: edit.editing;
        out property <bool> focused: edit.has-focus;
        out property <bool> other-focused: other.has-focus;
        public function focus-other() { other.focus(); }
        edit := InlineEdit {
            x: 10px;
            y: 10px;
            width: 200px;
            value: root.value;
            label: "Title";
            committed(text) => { root.value = text; root.commits += 1; }
        }
        other := FocusScope {
            x: 10px;
            y: 100px;
            width: 50px;
            height: 50px;
        }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    ely_slint::wire_fields!(h);
    show(&h);
    h
}

fn open(h: &Harness) {
    click(h.window(), 50.0, 20.0);
    advance(0);
    assert!(h.get_editing(), "a press opens the field");
    assert!(h.get_focused(), "the field takes focus");
}

fn a_press_opens_with_all_selected() {
    let h = harness();
    open(&h);
    // Everything is selected, so typing replaces it.
    tap(h.window(), "X");
    tap(h.window(), Key::Return);
    advance(0);
    assert_eq!(h.get_value(), "X");
}

fn enter_keeps_and_returns_focus() {
    let h = harness();
    open(&h);
    tap(h.window(), Key::End);
    tap(h.window(), "s");
    tap(h.window(), Key::Return);
    advance(0);
    assert!(!h.get_editing());
    assert_eq!(h.get_value(), "Drafts");
    assert_eq!(h.get_commits(), 1);
    assert!(h.get_focused(), "focus goes back to the text");
}

fn escape_reverts() {
    let h = harness();
    open(&h);
    tap(h.window(), "X");
    tap(h.window(), Key::Escape);
    advance(0);
    assert!(!h.get_editing());
    assert_eq!(h.get_value(), "Draft");
    assert_eq!(h.get_commits(), 0);
    assert!(h.get_focused());
}

fn leaving_keeps() {
    let h = harness();
    open(&h);
    tap(h.window(), "Y");
    h.invoke_focus_other();
    advance(0);
    assert!(!h.get_editing());
    assert_eq!(h.get_value(), "Y");
    assert!(h.get_other_focused(), "focus stays where it went");
}

fn keys_open_it() {
    let h = harness();
    tab(h.window());
    advance(0);
    assert!(h.get_focused());
    tap(h.window(), Key::Return);
    advance(0);
    assert!(h.get_editing(), "Enter on the text opens it");
}

cases!(inline_edit: a_press_opens_with_all_selected, enter_keeps_and_returns_focus, escape_reverts, leaving_keeps, keys_open_it);
