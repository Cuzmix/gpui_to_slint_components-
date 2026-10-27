// ConfirmButton, after ely-core/src/buttons/confirm.rs.
#[macro_use]
mod common;
use common::*;
use i_slint_backend_testing::{AccessibleRole, ElementHandle};
use slint::ComponentHandle;
use slint::platform::Key;

slint::slint! {
    import { ConfirmButton, ConfirmMode, Theme } from "ui/index.slint";
    export component Harness inherits Window {
        width: 400px;
        height: 120px;
        in-out property <int> twice-confirms;
        in-out property <int> hold-confirms;
        out property <bool> armed: twice.armed;
        out property <bool> done: twice.done;
        out property <bool> holding: hold.holding;
        out property <bool> hold-armed: hold.armed;
        out property <float> held: hold.hold-progress;
        out property <color> border: hold.current-border;
        out property <color> twice-fill: twice.current-fill;
        out property <color> c-danger: Theme.colors.danger;
        out property <color> c-hover: Theme.colors.hover;
        VerticalLayout {
            alignment: start;
            spacing: 8px;
            HorizontalLayout {
                alignment: start;
                twice := ConfirmButton {
                    text: "Delete project";
                    mode: ConfirmMode.twice;
                    confirmed => { root.twice-confirms += 1; }
                }
            }
            HorizontalLayout {
                alignment: start;
                hold := ConfirmButton {
                    text: "Erase disk";
                    armed-text: "Press once more";
                    mode: ConfirmMode.hold;
                    confirmed => { root.hold-confirms += 1; }
                }
            }
        }
    }
}

struct Box_ {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

fn button(h: &Harness, label: &str) -> Box_ {
    let found: Vec<_> = ElementHandle::find_by_accessible_label(h, label)
        .filter(|e| e.accessible_role() == Some(AccessibleRole::Button))
        .collect();
    assert_eq!(found.len(), 1, "one button named {label}");
    let (at, size) = (found[0].absolute_position(), found[0].size());
    Box_ { x: at.x, y: at.y, w: size.width, h: size.height }
}

fn mid(b: &Box_) -> (f32, f32) {
    (b.x + b.w / 2.0, b.y + b.h / 2.0)
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    show(&h);
    advance(16);
    h
}

// A first click arms it: it turns danger and asks again; a second confirms, shows done for
// 1.4 s (presses ignored), then reads its label again.
fn twice_arms_then_confirms() {
    let h = harness();
    let (x, y) = mid(&button(&h, "Delete project"));
    click(h.window(), x, y);
    assert!(h.get_armed());
    let (x, y) = mid(&button(&h, "Click again to confirm"));
    mouse_move(h.window(), 390.0, 110.0);
    assert_eq!(h.get_twice_fill(), h.get_c_danger(), "armed is the danger tone");
    click(h.window(), x, y);
    assert_eq!(h.get_twice_confirms(), 1);
    assert!(h.get_done() && !h.get_armed());
    click(h.window(), x, y);
    assert_eq!(h.get_twice_confirms(), 1, "done ignores presses");
    advance(1400);
    assert!(!h.get_done());
    assert_eq!(button(&h, "Delete project").w > 0.0, true);
}

// Armed, it waits 3 s, then disarms.
fn arming_runs_out_after_three_seconds() {
    let h = harness();
    let (x, y) = mid(&button(&h, "Delete project"));
    click(h.window(), x, y);
    advance(2990);
    assert!(h.get_armed());
    advance(20);
    assert!(!h.get_armed());
}

// Held 1.2 s, it confirms; the fill runs across meanwhile and the border turns danger. Let go
// early, nothing; a click alone does nothing in hold mode.
fn hold_confirms_after_a_full_hold() {
    let h = harness();
    let (x, y) = mid(&button(&h, "Erase disk"));
    click(h.window(), x, y);
    assert!(!h.get_hold_armed() && h.get_hold_confirms() == 0, "a click alone does nothing");
    mouse_move(h.window(), x, y);
    mouse_down(h.window(), x, y);
    assert!(h.get_holding());
    assert_eq!(h.get_border(), h.get_c_danger());
    advance(600);
    assert!((h.get_held() - 0.5).abs() < 0.02, "linear: {}", h.get_held());
    mouse_up(h.window(), x, y);
    assert!(!h.get_holding());
    advance(1000);
    assert_eq!(h.get_hold_confirms(), 0, "let go early");
    mouse_down(h.window(), x, y);
    advance(1210);
    assert_eq!(h.get_hold_confirms(), 1);
    assert!(!h.get_holding());
    mouse_up(h.window(), x, y);
    advance(1400);
}

// Keys always press twice, in hold mode too, with its own hint.
fn keys_press_twice() {
    let h = harness();
    tab(h.window());
    tab(h.window());
    tap(h.window(), Key::Return);
    assert!(h.get_hold_armed());
    assert!(!ElementHandle::find_by_accessible_label(&h, "Press once more").collect::<Vec<_>>().is_empty());
    tap(h.window(), " ");
    assert_eq!(h.get_hold_confirms(), 1);
    advance(1400);
}

cases!(confirm: twice_arms_then_confirms, arming_runs_out_after_three_seconds, hold_confirms_after_a_full_hold, keys_press_twice);
