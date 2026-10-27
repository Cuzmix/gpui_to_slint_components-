// Form, FormSection, FormField, FormLabel, FormError, InlineForm and DirtyIndicator, after
// ely-core/src/forms/structure.rs.
#[macro_use]
mod common;
use common::*;
use i_slint_backend_testing::{AccessibleRole, ElementHandle};
use slint::ComponentHandle;
use slint::platform::Key;

slint::slint! {
    import { Form, FormSection, FormField, InlineForm, DirtyIndicator, Forms, Input, Button, ButtonVariant, TextFields, TextFit } from "ui/index.slint";
    export { Forms, TextFields, TextFit }
    export component Harness inherits Window {
        width: 600px;
        height: 600px;
        in-out property <int> sent;
        in-out property <int> field-accepts;
        in-out property <string> error;
        in-out property <bool> dirty;
        out property <float> dirt-entrance: dirt.entrance;
        public function focus-name() { name.focus(); }
        public function focus-send() { send.focus(); }
        Form {
            x: 10px; y: 10px; width: 400px;
            owner: "signup";
            submitted => { root.sent += 1; }
            FormSection {
                title: "Profile";
                description: "How we reach you";
                f := FormField {
                    label: "Name";
                    required: true;
                    help: "As on your passport";
                    description: "We never share it.";
                    error: root.error;
                    name := Input { label: f.label; description: f.note; accepted => { root.field-accepts += 1; } }
                }
            }
            InlineForm {
                send := Button { text: "Send"; }
                dirt := DirtyIndicator { dirty: root.dirty; }
            }
        }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    ely_slint::wire_fields!(h);
    ely_slint::wire_forms!(h);
    show(&h);
    h
}

fn secondary_enter(h: &Harness) {
    key_down(h.window(), Key::Control);
    tap(h.window(), Key::Return);
    key_up(h.window(), Key::Control);
    advance(0);
}

fn texts(h: &Harness, text: &str) -> usize {
    ElementHandle::find_by_accessible_label(h, text).count()
}

fn secondary_enter_in_a_field_submits_the_form_once() {
    let h = harness();
    h.invoke_focus_name();
    tap(h.window(), "A");
    secondary_enter(&h);
    assert_eq!((h.get_sent(), h.get_field_accepts()), (1, 0));
    // Enter alone stays the field's.
    tap(h.window(), Key::Return);
    advance(0);
    assert_eq!((h.get_sent(), h.get_field_accepts()), (1, 1));
}

fn secondary_enter_on_any_control_in_a_form_submits_it() {
    let h = harness();
    h.invoke_focus_send();
    secondary_enter(&h);
    assert_eq!(h.get_sent(), 1, "the button holds focus");
}

fn a_field_shows_its_description_until_it_fails() {
    let h = harness();
    for text in ["Profile", "How we reach you", "Name", "*", "We never share it."] {
        assert!(texts(&h, text) > 0, "{text}");
    }
    let field = ElementHandle::find_by_accessible_label(&h, "Name").find(|e| e.accessible_role() == Some(AccessibleRole::TextInput)).expect("the input is named by its field");
    assert_eq!(field.accessible_description().as_deref(), Some("We never share it."));
    h.set_error("Enter your name.".into());
    advance(0);
    assert_eq!(texts(&h, "We never share it."), 0);
    assert_eq!(texts(&h, "Enter your name."), 1);
    assert_eq!(field.accessible_description().as_deref(), Some("Enter your name."));
}

fn the_dirty_mark_fades_in_while_unsaved() {
    let h = harness();
    assert_eq!(texts(&h, "Unsaved changes"), 0);
    h.set_dirty(true);
    advance(0);
    assert_eq!(texts(&h, "Unsaved changes"), 1);
    assert!(h.get_dirt_entrance() < 0.5);
    advance(250);
    assert_eq!(h.get_dirt_entrance(), 1.0);
    h.set_dirty(false);
    advance(0);
    assert_eq!(texts(&h, "Unsaved changes"), 0);
}

cases!(form_structure: secondary_enter_in_a_field_submits_the_form_once, secondary_enter_on_any_control_in_a_form_submits_it, a_field_shows_its_description_until_it_fails, the_dirty_mark_fades_in_while_unsaved);
