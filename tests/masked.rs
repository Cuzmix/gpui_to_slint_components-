// MaskedInput and PhoneInput, after ely-core/src/forms/masked.rs.
#[macro_use]
mod common;
use common::*;
use i_slint_backend_testing::{AccessibleRole, ElementHandle};
use slint::ComponentHandle;

slint::slint! {
    import { MaskedInput, PhoneInput, OptionLayer, TextFields, TextFit, Choices, Choice, Toggles } from "ui/index.slint";
    export { TextFields, TextFit, Choices, Choice, Toggles }
    export component Harness inherits Window {
        width: 400px;
        height: 400px;
        in-out property <string> zip;
        in-out property <string> card: "ab1234";
        in-out property <string> phone;
        in-out property <string> country: "US";
        out property <bool> picking: phone.picking;
        public function focus-zip() { zip.focus(); }
        public function focus-phone() { phone.focus(); }
        VerticalLayout {
            alignment: start;
            spacing: 8px;
            zip := MaskedInput { text <=> root.zip; mask: "(999) 999-9999"; }
            MaskedInput { text <=> root.card; mask: "aa-9999"; }
            phone := PhoneInput {
                text <=> root.phone;
                country: root.country;
                country-changed(code) => { root.country = code; }
            }
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

fn type_text(h: &Harness, text: &str) {
    for ch in text.chars() {
        tap(h.window(), ch.to_string());
    }
}

fn press(h: &Harness, element: &ElementHandle) {
    let (at, size) = (element.absolute_position(), element.size());
    click(h.window(), at.x + size.width / 2.0, at.y + size.height / 2.0);
}

fn labelled(h: &Harness, label: &str, role: AccessibleRole) -> Option<ElementHandle> {
    ElementHandle::find_by_accessible_label(h, label).find(|e| e.accessible_role() == Some(role))
}

fn typing_fills_the_slots() {
    let h = harness();
    h.invoke_focus_zip();
    type_text(&h, "555x1234567");
    assert_eq!(h.get_zip(), "(555) 123-4567");
    type_text(&h, "8");
    assert_eq!(h.get_zip(), "(555) 123-4567", "a full mask takes no more");
}

fn an_empty_field_shows_the_mask() {
    let h = harness();
    assert!(labelled(&h, "(___) ___-____", AccessibleRole::TextInput).is_some());
}

fn a_given_text_is_fitted() {
    let h = harness();
    advance(1);
    assert_eq!(h.get_card(), "ab-1234");
}

fn a_country_brings_its_pattern() {
    let h = harness();
    h.invoke_focus_phone();
    type_text(&h, "5551234567");
    assert_eq!(h.get_phone(), "(555) 123-4567");
    press(&h, &labelled(&h, "🇺🇸 +1", AccessibleRole::Button).expect("the country code"));
    advance(0);
    assert!(h.get_picking());
    let gb = labelled(&h, "🇬🇧  GB", AccessibleRole::ListItem).expect("the GB row");
    let us = labelled(&h, "🇺🇸  US", AccessibleRole::ListItem).unwrap();
    assert!(us.accessible_item_selected().unwrap_or(false), "the current country is checked");
    press(&h, &gb);
    advance(0);
    assert!(!h.get_picking());
    assert_eq!(h.get_country(), "GB");
    assert_eq!(h.get_phone(), "5551 234567");
    assert!(labelled(&h, "🇬🇧 +44", AccessibleRole::Button).is_some());
}

fn a_press_outside_closes_the_list() {
    let h = harness();
    press(&h, &labelled(&h, "🇺🇸 +1", AccessibleRole::Button).unwrap());
    advance(0);
    assert!(h.get_picking());
    click(h.window(), 390.0, 390.0);
    advance(0);
    assert!(!h.get_picking());
}

cases!(masked: typing_fills_the_slots, an_empty_field_shows_the_mask, a_given_text_is_fitted, a_country_brings_its_pattern, a_press_outside_closes_the_list);
