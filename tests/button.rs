#[macro_use]
mod common;
use common::*;
use slint::ComponentHandle;
use slint::platform::Key;

slint::slint! {
    import { Button, ButtonVariant, Theme } from "ui/index.slint";
    export component Harness inherits Window {
        width: 400px;
        height: 100px;
        in-out property <int> clicks;
        in-out property <bool> b-enabled: true;
        in-out property <bool> b-loading: false;
        out property <bool> first-focused: first.has-focus;
        out property <bool> second-focused: second.has-focus;
        out property <length> first-height: first.height;
        out property <color> fill: first.current-fill;
        out property <color> border: first.current-border;
        in-out property <ButtonVariant> variant: secondary;
        out property <color> c-surface: Theme.colors.surface;
        out property <color> c-hover: Theme.colors.hover;
        out property <color> c-active: Theme.colors.active;
        out property <color> c-border: Theme.colors.border;
        out property <color> c-focus: Theme.colors.focus;
        out property <color> c-accent: Theme.colors.accent;
        out property <color> c-accent-hover: Theme.colors.accent-hover;
        HorizontalLayout {
            alignment: start;
            first := Button {
                text: "Save";
                variant: variant;
                enabled: b-enabled;
                loading: b-loading;
                clicked => { clicks += 1; }
            }
            second := Button { text: "Other"; }
        }
    }
}

fn harness() -> Harness {
    init();
    let h = Harness::new().unwrap();
    show(&h);
    h
}

fn mouse_click_fires_and_does_not_take_focus() {
    let h = harness();
    click(h.window(), 10.0, 10.0);
    assert_eq!(h.get_clicks(), 1);
    assert!(!h.get_first_focused());
}

fn press_outside_release_inside_does_not_click() {
    let h = harness();
    mouse_move(h.window(), 390.0, 90.0);
    mouse_down(h.window(), 390.0, 90.0);
    mouse_move(h.window(), 10.0, 10.0);
    mouse_up(h.window(), 10.0, 10.0);
    assert_eq!(h.get_clicks(), 0);
}

fn disabled_ignores_press_and_leaves_tab_order() {
    let h = harness();
    h.set_b_enabled(false);
    click(h.window(), 10.0, 10.0);
    assert_eq!(h.get_clicks(), 0);
    tab(h.window());
    assert!(!h.get_first_focused());
    assert!(h.get_second_focused());
}

fn loading_ignores_press_but_keeps_tab_stop() {
    let h = harness();
    h.set_b_loading(true);
    click(h.window(), 10.0, 10.0);
    assert_eq!(h.get_clicks(), 0);
    tab(h.window());
    assert!(h.get_first_focused());
    key_down(h.window(), Key::Return);
    key_up(h.window(), Key::Return);
    assert_eq!(h.get_clicks(), 0);
}

fn enter_clicks_on_release_not_press() {
    let h = harness();
    tab(h.window());
    assert!(h.get_first_focused());
    key_down(h.window(), Key::Return);
    assert_eq!(h.get_clicks(), 0);
    key_up(h.window(), Key::Return);
    assert_eq!(h.get_clicks(), 1);
}

fn space_clicks_and_keys_are_interchangeable() {
    let h = harness();
    tab(h.window());
    tap(h.window(), " ");
    assert_eq!(h.get_clicks(), 1);
    // gpui: a Space press followed by an Enter release still clicks.
    key_down(h.window(), " ");
    key_up(h.window(), Key::Return);
    assert_eq!(h.get_clicks(), 2);
}

fn another_key_between_cancels() {
    let h = harness();
    tab(h.window());
    key_down(h.window(), Key::Return);
    key_down(h.window(), "a");
    key_up(h.window(), Key::Return);
    assert_eq!(h.get_clicks(), 0);
    key_down(h.window(), Key::Return);
    key_up(h.window(), "a");
    key_up(h.window(), Key::Return);
    assert_eq!(h.get_clicks(), 0);
}

fn modified_enter_does_not_click() {
    let h = harness();
    tab(h.window());
    key_down(h.window(), Key::Control);
    tap(h.window(), Key::Return);
    key_up(h.window(), Key::Control);
    assert_eq!(h.get_clicks(), 0);
}

fn focus_move_cancels_pending_key() {
    let h = harness();
    tab(h.window());
    key_down(h.window(), Key::Return);
    shift_tab(h.window());
    tab(h.window());
    key_up(h.window(), Key::Return);
    assert_eq!(h.get_clicks(), 0);
}

fn height_follows_size_and_density() {
    let h = harness();
    assert_eq!(h.get_first_height(), 28.0);
}

fn hover_and_press_fill_follow_the_tone() {
    let h = harness();
    assert_eq!(h.get_fill(), h.get_c_surface());
    assert_eq!(h.get_border(), h.get_c_border());
    mouse_move(h.window(), 10.0, 10.0);
    assert_eq!(h.get_fill(), h.get_c_hover());
    mouse_down(h.window(), 10.0, 10.0);
    assert_eq!(h.get_fill(), h.get_c_active());
    mouse_up(h.window(), 10.0, 10.0);
    assert_eq!(h.get_fill(), h.get_c_hover());
    mouse_move(h.window(), 390.0, 90.0);
    assert_eq!(h.get_fill(), h.get_c_surface());
}

fn primary_tone() {
    let h = harness();
    h.set_variant(ButtonVariant::Primary);
    assert_eq!(h.get_fill(), h.get_c_accent());
    mouse_move(h.window(), 10.0, 10.0);
    assert_eq!(h.get_fill(), h.get_c_accent_hover());
}

fn disabled_shows_no_hover() {
    let h = harness();
    h.set_b_enabled(false);
    mouse_move(h.window(), 10.0, 10.0);
    assert_eq!(h.get_fill(), h.get_c_surface());
}

fn focus_turns_the_border_focus_colored() {
    let h = harness();
    tab(h.window());
    assert_eq!(h.get_border(), h.get_c_focus());
}

mod flex {
    slint::slint! {
        import { Button } from "ui/index.slint";
        export component FlexHarness inherits Window {
            width: 300px;
            height: 100px;
            out property <length> tall-y: tall.y;
            out property <length> button-y: button.y;
            FlexboxLayout {
                cross-axis-alignment: center;
                cross-axis-line-alignment: start;
                tall := Rectangle { width: 20px; height: 40px; }
                button := Button { text: "Open"; shortcut: "Ctrl+O"; }
            }
        }
    }
}

// The ely-gallery row centers each item on the cross axis; a 28 px button beside a 40 px item sits 6 px down.
fn centers_in_a_flexbox_row() {
    init();
    let h = flex::FlexHarness::new().unwrap();
    show(&h);
    assert_eq!(h.get_tall_y(), 0.0);
    assert_eq!(h.get_button_y(), 6.0);
}

cases!(button: hover_and_press_fill_follow_the_tone, primary_tone, disabled_shows_no_hover, focus_turns_the_border_focus_colored, mouse_click_fires_and_does_not_take_focus, press_outside_release_inside_does_not_click, disabled_ignores_press_and_leaves_tab_order, loading_ignores_press_but_keeps_tab_stop, enter_clicks_on_release_not_press, space_clicks_and_keys_are_interchangeable, another_key_between_cancels, modified_enter_does_not_click, focus_move_cancels_pending_key, height_follows_size_and_density, centers_in_a_flexbox_row);
