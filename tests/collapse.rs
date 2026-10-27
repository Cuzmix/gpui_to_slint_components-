#[macro_use]
mod common;
use common::*;
use i_slint_backend_testing::ElementHandle;
use slint::platform::Key;

slint::slint! {
    import { Collapsible, Accordion, AccordionItem, Theme } from "ui/index.slint";
    export component Harness inherits Window {
        width: 600px;
        height: 800px;
        in-out property <bool> open: true;
        in-out property <bool> reduced <=> Theme.reduced-motion;
        out property <length> fold-h: fold.height;
        out property <bool> settled: fold.settled;
        in-out property <bool> multiple: false;
        out property <int> mask: faq.open-items;
        out property <length> faq-h: faq.preferred-height;
        out property <length> first-h: first.height;

        VerticalLayout {
            x: 0;
            y: 0;
            width: 420px;
            alignment: start;
            fold := Collapsible {
                open: root.open;
                Rectangle { height: 100px; }
            }
        }

        VerticalLayout {
            x: 0;
            y: 200px;
            width: 480px;
            alignment: start;
            faq := Accordion {
                first-open: true;
                multiple: root.multiple;
                first := AccordionItem { title: "What is Ely?"; text: "A component library."; open: faq.is-open(0); toggled => { faq.toggle(0); } }
                AccordionItem { title: "Does it animate?"; text: "Where motion explains a change."; open: faq.is-open(1); toggled => { faq.toggle(1); } }
                AccordionItem { title: "Can I theme it?"; text: "Every color comes from tokens."; open: faq.is-open(2); toggled => { faq.toggle(2); } }
            }
        }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    show(&h);
    advance(16);
    h
}

fn ease_in_out_cubic(t: f32) -> f32 {
    if t < 0.5 { 4.0 * t * t * t } else { 1.0 - (-2.0 * t + 2.0).powi(3) / 2.0 }
}

fn a_collapsible_starts_still_then_folds() {
    let h = harness();
    assert_eq!(h.get_fold_h(), 100.0);
    h.set_open(false);
    advance(0);
    assert_eq!(h.get_fold_h(), 100.0);
    advance(50);
    assert!((h.get_fold_h() - 100.0 * (1.0 - ease_in_out_cubic(0.25))).abs() < 0.5, "{}", h.get_fold_h());
    advance(160);
    assert!(h.get_settled());
    assert_eq!(h.get_fold_h(), 0.0);
}

// Toggled mid-way, it starts over from the far end, as gpui does.
fn a_change_mid_fold_starts_from_the_far_end() {
    let h = harness();
    h.set_open(false);
    advance(0);
    advance(100);
    assert!(h.get_fold_h() < 60.0 && h.get_fold_h() > 40.0);
    h.set_open(true);
    advance(0);
    assert!(h.get_fold_h() < 1.0, "opening grows from nothing: {}", h.get_fold_h());
    advance(210);
    assert_eq!(h.get_fold_h(), 100.0);
}

fn reduced_motion_folds_at_once() {
    let h = harness();
    h.set_reduced(true);
    h.set_open(false);
    advance(0);
    advance(5);
    assert_eq!(h.get_fold_h(), 0.0);
    h.set_reduced(false);
}

fn one_item_open_at_a_time() {
    let h = harness();
    assert_eq!(h.get_mask(), 1);
    let header = |title: &str| ElementHandle::find_by_accessible_label(&h, title).next().unwrap();
    let second = header("Does it animate?");
    let at = second.absolute_position();
    click(h.window(), at.x + 10.0, at.y + 10.0);
    assert_eq!(h.get_mask(), 2);
    click(h.window(), at.x + 10.0, at.y + 10.0);
    assert_eq!(h.get_mask(), 0, "shutting the open one leaves none");
    h.set_multiple(true);
    click(h.window(), at.x + 10.0, at.y + 10.0);
    let third = header("Can I theme it?").absolute_position();
    advance(300);
    click(h.window(), third.x + 10.0, third.y + 10.0);
    assert_eq!(h.get_mask(), 2 + 4);
}

// Header: 1 + 12 + base text (13 px: 21) + 12 + 1 = 47, plus the 1 px line under each item.
fn items_measure_like_gpui() {
    let h = harness();
    // The first is open: its body is 21 + 16 below the header.
    assert_eq!(h.get_first_h(), 47.0 + 21.0 + 16.0 + 1.0);
    assert_eq!(h.get_faq_h(), 1.0 + (47.0 + 21.0 + 16.0 + 1.0) + 2.0 * (47.0 + 1.0));
}

fn the_header_is_an_expandable_button() {
    let h = harness();
    let first = ElementHandle::find_by_accessible_label(&h, "What is Ely?").next().unwrap();
    assert_eq!(first.accessible_role(), Some(i_slint_backend_testing::AccessibleRole::Button));
    assert_eq!(first.accessible_expanded(), Some(true));
    tab(h.window());
    tap(h.window(), Key::Return);
    assert_eq!(h.get_mask(), 0);
    tap(h.window(), " ");
    assert_eq!(h.get_mask(), 1);
}

cases!(collapse: a_collapsible_starts_still_then_folds, a_change_mid_fold_starts_from_the_far_end, reduced_motion_folds_at_once, one_item_open_at_a_time, items_measure_like_gpui, the_header_is_an_expandable_button);
