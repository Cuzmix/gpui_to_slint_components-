// TagInput, after ely-core/src/forms/tags.rs.
#[macro_use]
mod common;
use common::*;
use i_slint_backend_testing::{AccessibleRole, ElementHandle};
use slint::platform::Key;
use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel};

slint::slint! {
    import { TagInput, OptionLayer, TextFields, TextFit, Choices, Choice, Toggles } from "ui/index.slint";
    export { TextFields, TextFit, Choices, Choice, Toggles }
    export component Harness inherits Window {
        width: 400px;
        height: 300px;
        in-out property <[string]> tags: ["rust"];
        in-out property <string> last-typed;
        out property <bool> open: input.open;
        out property <int> highlighted: input.highlighted;
        out property <string> typed: input.typed;
        out property <bool> unaccepted: input.unaccepted;
        public function focus-tags() { input.focus(); }
        public function focus-other() { other.focus(); }
        VerticalLayout {
            alignment: start;
            input := TagInput {
                tags: root.tags;
                suggestions: [
                    { value: "rs", label: "Rust (lang)" },
                    { value: "ruby", label: "Ruby" },
                    { value: "go", label: "Go" },
                ];
                check(tag) => { tag != "bad" }
                tags-changed(next) => { root.tags = next; }
                typing(text) => { root.last-typed = text; }
            }
            other := FocusScope { height: 20px; }
        }
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

fn tags(h: &Harness) -> Vec<String> {
    h.get_tags().iter().map(|tag| tag.to_string()).collect()
}

fn type_text(h: &Harness, text: &str) {
    for ch in text.chars() {
        tap(h.window(), ch.to_string());
    }
}

fn enter_and_commas_add() {
    let h = harness();
    h.invoke_focus_tags();
    type_text(&h, "gpui");
    tap(h.window(), Key::Return);
    assert_eq!(tags(&h), ["rust", "gpui"]);
    assert_eq!(h.get_typed(), "");
    type_text(&h, " slint ,");
    assert_eq!(tags(&h), ["rust", "gpui", "slint"]);
    type_text(&h, "\"a,");
    assert_eq!(tags(&h).len(), 3, "a comma inside quotes stays");
    assert_eq!(h.get_last_typed(), "\"a,");
}

fn duplicates_and_blanks_clear_without_adding() {
    let h = harness();
    h.invoke_focus_tags();
    type_text(&h, "rust,");
    assert_eq!(tags(&h), ["rust"]);
    assert_eq!(h.get_typed(), "");
    type_text(&h, "  ,");
    assert_eq!(tags(&h), ["rust"]);
}

fn backspace_in_an_empty_field_drops_the_last() {
    let h = harness();
    h.set_tags(ModelRc::new(VecModel::from(vec![SharedString::from("a"), "b".into()])));
    h.invoke_focus_tags();
    tap(h.window(), Key::Backspace);
    assert_eq!(tags(&h), ["a"]);
}

fn a_chip_removes_itself() {
    let h = harness();
    let remove = ElementHandle::find_by_accessible_label(&h, "Remove rust")
        .find(|e| e.accessible_role() == Some(AccessibleRole::Button))
        .expect("the chip's remove button");
    remove.invoke_accessible_default_action();
    assert!(tags(&h).is_empty());
}

fn suggestions_float_and_add_their_value() {
    let h = harness();
    h.invoke_focus_tags();
    type_text(&h, "ru");
    advance(0);
    assert!(h.get_open());
    let rows: Vec<_> = ElementHandle::find_by_element_type_name(&h, "OptionRow").collect();
    assert_eq!(rows.len(), 2, "Rust (lang) and Ruby; Go does not hold \"ru\"");
    tap(h.window(), Key::DownArrow);
    assert_eq!(h.get_highlighted(), 1);
    tap(h.window(), Key::Return);
    assert_eq!(tags(&h), ["rust", "ruby"]);
    type_text(&h, "lang");
    advance(0);
    tap(h.window(), Key::Return);
    assert_eq!(tags(&h), ["rust", "ruby", "rs"]);
    assert!(
        ElementHandle::find_by_accessible_label(&h, "Rust (lang)").any(|e| e.accessible_role() == Some(AccessibleRole::Text)),
        "a chip shows its suggestion's label"
    );
}

fn escape_hides_until_the_next_key() {
    let h = harness();
    h.invoke_focus_tags();
    type_text(&h, "g");
    advance(0);
    assert!(h.get_open());
    tap(h.window(), Key::Escape);
    advance(0);
    assert!(!h.get_open());
    type_text(&h, "o");
    advance(0);
    assert!(h.get_open());
}

fn a_failing_word_shows_once_focus_leaves() {
    let h = harness();
    h.invoke_focus_tags();
    type_text(&h, "bad");
    assert!(!h.get_unaccepted());
    h.invoke_focus_other();
    assert!(h.get_unaccepted());
}

cases!(tag_input: enter_and_commas_add, duplicates_and_blanks_clear_without_adding, backspace_in_an_empty_field_drops_the_last, a_chip_removes_itself, suggestions_float_and_add_their_value, escape_hides_until_the_next_key, a_failing_word_shows_once_focus_leaves);
