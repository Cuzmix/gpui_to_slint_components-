// NumberInput and ScrubInput, after ely-core/src/forms/number.rs and number/tests.rs.
#[macro_use]
mod common;
use common::*;
use slint::ComponentHandle;
use slint::platform::Key;

slint::slint! {
    import { NumberInput, ScrubInput, Numbers, ParsedNumber, TextFields, TextFit } from "ui/index.slint";
    export { Numbers, ParsedNumber, TextFields, TextFit }
    export component Harness inherits Window {
        width: 500px;
        height: 200px;
        in-out property <float> qty: 3;
        in-out property <float> price: 1234.5;
        in-out property <float> width-value: 240;
        in-out property <int> changes;
        in-out property <int> commits;
        in-out property <float> last-commit: -1;
        out property <string> qty-text: qty.text;
        out property <string> price-text: price.text;
        out property <bool> can-up: qty.can-step-up;
        out property <bool> can-down: qty.can-step-down;
        out property <bool> qty-focused: qty.has-focus;
        public function focus-qty() { qty.focus(); }
        public function focus-price() { price.focus(); }
        VerticalLayout {
            alignment: start;
            spacing: 8px;
            qty := NumberInput {
                value <=> root.qty;
                minimum: 1;
                maximum: 10;
                label: "Quantity";
                value-changed => { root.changes += 1; }
                committed(v) => { root.commits += 1; root.last-commit = v; }
            }
            price := NumberInput {
                value <=> root.price;
                currency: "USD";
            }
            ScrubInput {
                label: "W";
                value <=> root.width-value;
                minimum: 0;
                maximum: 4096;
            }
        }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    ely_slint::wire_fields!(h);
    ely_slint::wire_numbers!(h);
    show(&h);
    h
}

fn type_text(h: &Harness, text: &str) {
    for ch in text.chars() {
        tap(h.window(), ch.to_string());
    }
}

fn select_all(h: &Harness) {
    key_down(h.window(), Key::Control);
    tap(h.window(), "a");
    key_up(h.window(), Key::Control);
}

fn it_shows_its_value() {
    let h = harness();
    assert_eq!(h.get_qty_text(), "3");
    assert_eq!(h.get_price_text(), "1,234.50", "a currency shows its minor units");
}

fn typing_reports_settled_numbers_only() {
    let h = harness();
    h.invoke_focus_qty();
    select_all(&h);
    type_text(&h, "7");
    assert_eq!(h.get_qty(), 7.0);
    assert_eq!(h.get_changes(), 1);
    assert_eq!(h.get_commits(), 0, "typing never commits");
    type_text(&h, "0");
    assert_eq!(h.get_qty_text(), "70");
    assert_eq!(h.get_qty(), 7.0, "70 is out of range: not reported");
    tap(h.window(), Key::Return);
    assert_eq!(h.get_qty(), 10.0);
    assert_eq!(h.get_qty_text(), "10");
    assert_eq!(h.get_last_commit(), 10.0);
}

fn letters_never_go_in() {
    let h = harness();
    h.invoke_focus_qty();
    select_all(&h);
    type_text(&h, "a5b");
    assert_eq!(h.get_qty_text(), "5");
}

fn leaving_the_field_settles_it() {
    let h = harness();
    h.invoke_focus_price();
    select_all(&h);
    type_text(&h, "2.345");
    h.invoke_focus_qty();
    advance(0);
    assert_eq!(h.get_price_text(), "2.35");
    assert!((h.get_price() - 2.35).abs() < 1e-4);
}

fn arrows_step_and_commit() {
    let h = harness();
    h.invoke_focus_qty();
    tap(h.window(), Key::UpArrow);
    assert_eq!(h.get_qty(), 4.0);
    assert_eq!(h.get_commits(), 1);
    tap(h.window(), Key::DownArrow);
    tap(h.window(), Key::DownArrow);
    assert_eq!(h.get_qty(), 2.0);
    assert_eq!(h.get_qty_text(), "2");
}

fn the_stepper_stops_at_the_ends() {
    let h = harness();
    h.set_qty(10.0);
    assert!(!h.get_can_up());
    assert!(h.get_can_down());
    h.set_qty(1.0);
    assert!(h.get_can_up());
    assert!(!h.get_can_down());
}

fn a_new_value_shows_unless_editing() {
    let h = harness();
    h.set_qty(5.0);
    advance(0);
    assert_eq!(h.get_qty_text(), "5");
}

fn dragging_the_label_scrubs() {
    let h = harness();
    let label = i_slint_backend_testing::ElementHandle::find_by_accessible_label(&h, "W")
        .find(|e| e.accessible_role() == Some(i_slint_backend_testing::AccessibleRole::Text))
        .expect("the scrub label");
    let at = label.absolute_position();
    let (x, y) = (at.x + 4.0, at.y + 4.0);
    mouse_move(h.window(), x, y);
    mouse_down(h.window(), x, y);
    mouse_move(h.window(), x + 1.0, y);
    mouse_move(h.window(), x + 1.0 + 13.0, y);
    // 13 px at 6 px a notch: two steps.
    assert_eq!(h.get_width_value(), 242.0);
    mouse_move(h.window(), x + 1.0 - 7.0, y);
    assert_eq!(h.get_width_value(), 239.0);
    mouse_up(h.window(), x, y);
}

cases!(number: it_shows_its_value, typing_reports_settled_numbers_only, letters_never_go_in, leaving_the_field_settles_it, arrows_step_and_commit, the_stepper_stops_at_the_ends, a_new_value_shows_unless_editing, dragging_the_label_scrubs);
