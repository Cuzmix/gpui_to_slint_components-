#[macro_use]
mod common;
use common::*;
use i_slint_backend_testing::ElementHandle;
use slint::ComponentHandle;
use slint::platform::Key;
use std::{cell::RefCell, rc::Rc};

slint::slint! {
    import { Theme, TypeText, Link, ExternalLink, Links } from "ui/index.slint";
    export { Links }
    export component Harness inherits Window {
        width: 400px;
        height: 200px;
        in-out property <int> presses;
        out property <length> link-w: guide.width;
        out property <length> link-h: guide.height;
        out property <length> label-w: probe.preferred-width;
        out property <length> site-w: site.width;
        out property <bool> guide-focused: guide.has-focus;
        out property <bool> ring-rest: guide.current-border == transparent;
        out property <bool> ring-focus: guide.current-border == Theme.colors.focus;
        out property <bool> site-focused: site.has-focus;
        probe := TypeText { visible: false; text: "style guide"; wrap: no-wrap; }
        guide := Link { x: 10px; y: 10px; text: "style guide"; clicked => { root.presses += 1; } }
        site := ExternalLink { x: 10px; y: 50px; text: "gpui.rs"; url: "https://www.gpui.rs"; }
    }
}

fn harness() -> (Harness, Rc<RefCell<Vec<String>>>) {
    let h = Harness::new().unwrap();
    let opened = Rc::new(RefCell::new(Vec::new()));
    let seen = opened.clone();
    h.global::<Links>().on_open(move |url| seen.borrow_mut().push(url.to_string()));
    show(&h);
    (h, opened)
}

fn a_link_hugs_its_text_inside_a_ring() {
    let (h, _) = harness();
    // 2 px padding in a 1 px ring across; base text at phi is 21 px, plus the ring.
    assert!((h.get_link_w() - (h.get_label_w() + 6.0)).abs() < 0.01);
    assert_eq!(h.get_link_h(), 23.0);
    // The external arrow: 2 px gap and a 12 px icon.
    let site_probe = h.get_site_w();
    assert!(site_probe > 14.0);
}

fn a_press_runs_it_without_taking_focus() {
    let (h, _) = harness();
    click(h.window(), 20.0, 20.0);
    assert_eq!(h.get_presses(), 1);
    assert!(!h.get_guide_focused());
}

fn enter_and_space_run_it_on_release() {
    let (h, opened) = harness();
    tab(h.window());
    assert!(h.get_guide_focused());
    key_down(h.window(), Key::Return);
    assert_eq!(h.get_presses(), 0);
    key_up(h.window(), Key::Return);
    assert_eq!(h.get_presses(), 1);
    tap(h.window(), " ");
    assert_eq!(h.get_presses(), 2);
    tab(h.window());
    assert!(h.get_site_focused());
    tap(h.window(), Key::Return);
    assert_eq!(opened.borrow().as_slice(), ["https://www.gpui.rs"]);
    assert_eq!(h.get_presses(), 2);
}

fn the_ring_turns_focus_colored() {
    let (h, _) = harness();
    assert!(h.get_ring_rest());
    tab(h.window());
    assert!(h.get_ring_focus());
}

fn hover_underlines_the_text() {
    let (h, _) = harness();
    let underlines = |h: &Harness| {
        ElementHandle::find_by_element_type_name(h, "Rectangle")
            .filter(|e| e.size().height == 1.0 && e.absolute_position().y < 40.0)
            .count()
    };
    assert_eq!(underlines(&h), 0);
    mouse_move(h.window(), 20.0, 20.0);
    assert_eq!(underlines(&h), 1);
    mouse_move(h.window(), 300.0, 150.0);
    assert_eq!(underlines(&h), 0);
}

fn the_link_reads_as_pressable() {
    let (h, _) = harness();
    let link = ElementHandle::find_by_accessible_label(&h, "style guide")
        .find(|e| e.accessible_role() == Some(i_slint_backend_testing::AccessibleRole::Button))
        .expect("a pressable node");
    link.invoke_accessible_default_action();
    assert_eq!(h.get_presses(), 1);
}

cases!(link: a_link_hugs_its_text_inside_a_ring, a_press_runs_it_without_taking_focus, enter_and_space_run_it_on_release, the_ring_turns_focus_colored, hover_underlines_the_text, the_link_reads_as_pressable);
