// TextField, Input and PasswordInput, after ely-core/src/forms/{text,input}.rs.
#[macro_use]
mod common;
use common::*;
use i_slint_backend_testing::{AccessibleRole, ElementHandle};
use slint::ComponentHandle;
use slint::platform::Key;

slint::slint! {
    import { Input, PasswordInput, TextFields, TextFit, Theme, ControlSize, TextSize, Icons } from "ui/index.slint";
    export { TextFields, TextFit }
    export component Harness inherits Window {
        width: 400px;
        height: 400px;
        in-out property <string> plain;
        in-out property <string> secret: "hunter2";
        in-out property <string> notes;
        in-out property <int> edits;
        in-out property <int> accepts;
        in-out property <int> note-accepts;
        in-out property <int> max-len: 0;
        in-out property <string> kind;
        in-out property <bool> invalid;
        in-out property <ControlSize> size: ControlSize.md;
        out property <length> plain-height: plain.height;
        out property <length> notes-height: notes.height;
        out property <brush> plain-border: plain.current-border;
        out property <bool> plain-focused: plain.has-focus;
        out property <bool> masked: secret.masked;
        out property <color> focus-color: Theme.colors.focus;
        out property <color> danger-color: Theme.colors.danger;
        out property <color> strong-color: Theme.colors.border-strong;
        out property <length> control-sm: Theme.control-height(ControlSize.sm);
        out property <length> control-lg: Theme.control-height(ControlSize.lg);
        out property <length> base-size: Theme.text-size(TextSize.base);
        public function focus-plain() { plain.focus(); }
        public function focus-notes() { notes.focus(); }
        VerticalLayout {
            alignment: start;
            spacing: 8px;
            plain := Input {
                text <=> root.plain;
                placeholder: "Search";
                prefix-icon: Icons.search;
                clearable: true;
                invalid: root.invalid;
                size: root.size;
                max-len: root.max-len;
                fit-kind: root.kind;
                edited => { root.edits += 1; }
                accepted => { root.accepts += 1; }
            }

            secret := PasswordInput {
                text <=> root.secret;
                label: "Password";
            }

            notes := Input {
                text <=> root.notes;
                placeholder: "Notes";
                rows-min: 2;
                rows-max: 4;
                accepted => { root.note-accepts += 1; }
            }
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

fn labelled(h: &Harness, label: &str) -> Vec<ElementHandle> {
    ElementHandle::find_by_accessible_label(h, label).collect()
}

fn typing_edits_the_text() {
    let h = harness();
    h.invoke_focus_plain();
    type_text(&h, "ely");
    assert_eq!(h.get_plain(), "ely");
    assert_eq!(h.get_edits(), 3);
}

fn the_placeholder_names_an_empty_field() {
    let h = harness();
    let field = labelled(&h, "Search").into_iter().find(|e| e.accessible_role() == Some(AccessibleRole::TextInput));
    assert!(field.is_some(), "the placeholder names the field");
}

fn enter_accepts_one_line() {
    let h = harness();
    h.invoke_focus_plain();
    tap(h.window(), Key::Return);
    assert_eq!(h.get_accepts(), 1);
}

fn several_lines_break_and_ctrl_enter_accepts() {
    let h = harness();
    h.invoke_focus_notes();
    type_text(&h, "a");
    tap(h.window(), Key::Return);
    type_text(&h, "b");
    assert_eq!(h.get_notes(), "a\nb");
    assert_eq!(h.get_note_accepts(), 0);
    key_down(h.window(), Key::Control);
    tap(h.window(), Key::Return);
    key_up(h.window(), Key::Control);
    assert_eq!(h.get_note_accepts(), 1);
    assert_eq!(h.get_notes(), "a\nb");
}

fn several_lines_grow_from_least_to_most_rows() {
    let h = harness();
    // A phi line of base text, snapped; 6 px and the border above and below.
    let line = (1.618034f32 * h.get_base_size() - 0.5).ceil();
    let at_least = h.get_notes_height();
    assert_eq!(at_least, line * 2.0 + 14.0);
    h.set_notes("1\n2\n3".into());
    assert_eq!(h.get_notes_height(), line * 3.0 + 14.0);
    h.set_notes("1\n2\n3\n4\n5\n6".into());
    assert_eq!(h.get_notes_height(), line * 4.0 + 14.0);
}

fn the_frame_is_a_control_tall() {
    let h = harness();
    h.set_size(ControlSize::Sm);
    assert_eq!(h.get_plain_height(), h.get_control_sm());
    h.set_size(ControlSize::Lg);
    assert_eq!(h.get_plain_height(), h.get_control_lg());
}

fn the_border_shows_focus_and_danger() {
    let h = harness();
    assert_eq!(h.get_plain_border(), h.get_strong_color().into());
    h.invoke_focus_plain();
    assert_eq!(h.get_plain_border(), h.get_focus_color().into());
    h.set_invalid(true);
    assert_eq!(h.get_plain_border(), h.get_danger_color().into());
}

fn clearing_empties_and_keeps_focus() {
    let h = harness();
    assert!(labelled(&h, "Clear").is_empty(), "no clear button while empty");
    h.invoke_focus_plain();
    type_text(&h, "xy");
    let clear = labelled(&h, "Clear");
    assert_eq!(clear.len(), 1);
    clear[0].invoke_accessible_default_action();
    assert_eq!(h.get_plain(), "");
    assert!(h.get_plain_focused());
    assert!(labelled(&h, "Clear").is_empty());
}

fn max_len_caps_characters() {
    let h = harness();
    h.set_max_len(4);
    h.invoke_focus_plain();
    type_text(&h, "abcdef");
    assert_eq!(h.get_plain(), "abcd");
}

fn a_filter_drops_characters() {
    ely_slint::fields::register_filter("digits", |ch| ch.is_ascii_digit());
    let h = harness();
    h.set_kind("digits".into());
    h.invoke_focus_plain();
    type_text(&h, "1a2b3");
    assert_eq!(h.get_plain(), "123");
    type_text(&h, "4");
    assert_eq!(h.get_plain(), "1234", "the caret stays after what was kept");
}

fn the_eye_shows_and_hides_the_password() {
    let h = harness();
    assert!(h.get_masked());
    let show = labelled(&h, "Show password");
    assert_eq!(show.len(), 1);
    show[0].invoke_accessible_default_action();
    assert!(!h.get_masked());
    let hide = labelled(&h, "Hide password");
    assert_eq!(hide.len(), 1);
    hide[0].invoke_accessible_default_action();
    assert!(h.get_masked());
    assert_eq!(h.get_secret(), "hunter2");
}

cases!(input: typing_edits_the_text, the_placeholder_names_an_empty_field, enter_accepts_one_line, several_lines_break_and_ctrl_enter_accepts, several_lines_grow_from_least_to_most_rows, the_frame_is_a_control_tall, the_border_shows_focus_and_danger, clearing_empties_and_keeps_focus, max_len_caps_characters, a_filter_drops_characters, the_eye_shows_and_hides_the_password);
