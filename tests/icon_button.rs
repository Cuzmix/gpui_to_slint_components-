#[macro_use]
mod common;
use common::*;
use slint::ComponentHandle;
use slint::platform::Key;

slint::slint! {
    import { IconButton, CloseButton, Icons, ControlSize, Theme } from "ui/index.slint";
    export component Harness inherits Window {
        width: 300px;
        height: 200px;
        in-out property <int> clicks;
        in-out property <bool> b-enabled: true;
        in-out property <ControlSize> b-size: md;
        out property <bool> shown: btn.tooltip-shown;
        out property <length> side: btn.width;
        out property <color> fill: btn.current-fill;
        out property <color> c-hover: Theme.colors.hover;
        out property <bool> focused: btn.has-focus;
        btn := IconButton {
            x: 0;
            y: 0;
            icon: Icons.settings;
            tooltip: "Settings";
            enabled: b-enabled;
            size: b-size;
            clicked => { clicks += 1; }
        }
        close := CloseButton { x: 100px; y: 0; }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    show(&h);
    h
}

fn square_at_control_height() {
    let h = harness();
    assert_eq!(h.get_side(), 28.0);
    h.set_b_size(ControlSize::Sm);
    assert_eq!(h.get_side(), 24.0);
    h.set_b_size(ControlSize::Lg);
    assert_eq!(h.get_side(), 32.0);
}

fn clicks_and_keyboard() {
    let h = harness();
    click(h.window(), 10.0, 10.0);
    assert_eq!(h.get_clicks(), 1);
    assert!(!h.get_focused());
    tab(h.window());
    assert!(h.get_focused());
    tap(h.window(), Key::Return);
    assert_eq!(h.get_clicks(), 2);
}

fn ghost_hover_fill() {
    let h = harness();
    mouse_move(h.window(), 10.0, 10.0);
    assert_eq!(h.get_fill(), h.get_c_hover());
}

fn tooltip_shows_after_500ms_of_hover() {
    let h = harness();
    mouse_move(h.window(), 10.0, 10.0);
    advance(499);
    assert!(!h.get_shown());
    advance(1);
    assert!(h.get_shown());
    mouse_move(h.window(), 250.0, 150.0);
    assert!(!h.get_shown());
}

fn mouse_down_hides_the_tooltip_until_the_pointer_moves_again() {
    let h = harness();
    mouse_move(h.window(), 10.0, 10.0);
    advance(500);
    assert!(h.get_shown());
    mouse_down(h.window(), 10.0, 10.0);
    assert!(!h.get_shown());
    mouse_up(h.window(), 10.0, 10.0);
    advance(600);
    assert!(!h.get_shown());
    mouse_move(h.window(), 11.0, 10.0);
    advance(500);
    assert!(h.get_shown());
}

fn disabled_keeps_its_tooltip_but_not_clicks() {
    let h = harness();
    h.set_b_enabled(false);
    click(h.window(), 10.0, 10.0);
    assert_eq!(h.get_clicks(), 0);
    mouse_move(h.window(), 12.0, 10.0);
    advance(500);
    assert!(h.get_shown());
}

fn close_button_reads_close() {
    let h = harness();
    let found: Vec<_> = i_slint_backend_testing::ElementHandle::find_by_accessible_label(&h, "Close").collect();
    assert!(!found.is_empty());
    let found: Vec<_> = i_slint_backend_testing::ElementHandle::find_by_accessible_label(&h, "Settings").collect();
    assert!(!found.is_empty());
    assert_eq!(found[0].accessible_role(), Some(i_slint_backend_testing::AccessibleRole::Button));
}

cases!(icon_button: square_at_control_height, clicks_and_keyboard, ghost_hover_fill, tooltip_shows_after_500ms_of_hover, mouse_down_hides_the_tooltip_until_the_pointer_moves_again, disabled_keeps_its_tooltip_but_not_clicks, close_button_reads_close);
