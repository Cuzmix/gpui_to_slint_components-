// FloatingActionButton, after ely-core/src/buttons/fab.rs.
#[macro_use]
mod common;
use common::*;
use i_slint_backend_testing::{AccessibleRole, ElementHandle};
use slint::ComponentHandle;
use slint::platform::Key;

slint::slint! {
    import { FloatingActionButton, Elevation, Icons, Theme, TextSize } from "ui/index.slint";
    export { Elevation }
    export component Harness inherits Window {
        width: 400px;
        height: 200px;
        in-out property <int> clicks;
        out property <length> round-width: round.width;
        out property <length> round-height: round.height;
        out property <length> pill-width: pill.width;
        out property <length> word: probe.preferred-width;
        out property <color> fill: round.current-fill;
        out property <Elevation> lift: round.current-elevation;
        out property <float> entrance: round.entrance;
        out property <bool> focused: round.has-focus;
        out property <bool> tip: round.tooltip-shown;
        out property <color> c-accent: Theme.colors.accent;
        out property <color> c-accent-hover: Theme.colors.accent-hover;
        round := FloatingActionButton {
            x: 0;
            y: 0;
            icon: Icons.pencil;
            tooltip: "Compose";
            clicked => { root.clicks += 1; }
        }
        pill := FloatingActionButton {
            x: 100px;
            y: 0;
            icon: Icons.plus;
            text: "New task";
        }
        probe := Text {
            visible: false;
            text: "New task";
            font-size: Theme.text-size(TextSize.md);
            font-weight: 500;
            font-family: Theme.font-family;
        }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    show(&h);
    h
}

// Round at 56 px; a label makes a pill 20 px before the icon and 24 px after the text, inside a
// 1 px border, the icon and text 8 px apart.
fn round_or_a_pill() {
    let h = harness();
    assert_eq!((h.get_round_width(), h.get_round_height()), (56.0, 56.0));
    assert_eq!(h.get_pill_width(), 2.0 + 20.0 + 20.0 + 8.0 + h.get_word() + 24.0);
}

// It fades in from 8 px lower over the slow motion.
fn it_rises_in() {
    let h = harness();
    advance(0);
    assert!(h.get_entrance() < 0.01, "starts hidden: {}", h.get_entrance());
    advance(160);
    assert!(h.get_entrance() > 0.5 && h.get_entrance() < 1.0, "eases out: {}", h.get_entrance());
    advance(200);
    assert_eq!(h.get_entrance(), 1.0);
}

// Hover lifts it and darkens the accent; a press clicks without taking focus; the tooltip names it.
fn hover_lifts_and_a_press_clicks() {
    let h = harness();
    advance(400);
    assert_eq!(h.get_fill(), h.get_c_accent());
    assert_eq!(h.get_lift(), Elevation::Floating);
    mouse_move(h.window(), 28.0, 28.0);
    assert_eq!(h.get_fill(), h.get_c_accent_hover());
    assert_eq!(h.get_lift(), Elevation::Modal);
    advance(600);
    assert!(h.get_tip());
    click(h.window(), 28.0, 28.0);
    assert_eq!(h.get_clicks(), 1);
    assert!(!h.get_focused());
    let named: Vec<_> = ElementHandle::find_by_accessible_label(&h, "Compose")
        .filter(|e| e.accessible_role() == Some(AccessibleRole::Button))
        .collect();
    assert_eq!(named.len(), 1);
    assert_eq!(
        ElementHandle::find_by_accessible_label(&h, "New task")
            .filter(|e| e.accessible_role() == Some(AccessibleRole::Button))
            .count(),
        1,
        "a pill is named by its text"
    );
}

// Tab reaches it; Enter clicks on release.
fn keys_click() {
    let h = harness();
    tab(h.window());
    assert!(h.get_focused());
    key_down(h.window(), Key::Return);
    assert_eq!(h.get_clicks(), 0);
    key_up(h.window(), Key::Return);
    assert_eq!(h.get_clicks(), 1);
}

cases!(fab: round_or_a_pill, it_rises_in, hover_lifts_and_a_press_clicks, keys_click);
