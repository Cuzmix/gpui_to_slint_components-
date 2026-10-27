// ActionBar and BulkActionBar, after ely-core/src/buttons/bars.rs.
#[macro_use]
mod common;
use common::*;
use i_slint_backend_testing::{AccessibleRole, ElementHandle};
use slint::ComponentHandle;

slint::slint! {
    import { ActionBar, BulkActionBar, Button, ButtonVariant } from "ui/index.slint";
    export component Harness inherits Window {
        width: 500px;
        height: 300px;
        in-out property <int> selected;
        in-out property <int> clears;
        out property <length> docked-height: docked.height;
        out property <length> pill-height: pill.height;
        out property <length> pill-width: pill.width;
        out property <length> bulk-height: bulk.height;
        out property <float> docked-entrance: docked.entrance;
        out property <float> bulk-entrance: bulk.entrance;
        VerticalLayout {
            alignment: start;
            spacing: 8px;
            docked := ActionBar {
                Button { text: "Cancel"; }
                Button { text: "Save"; variant: ButtonVariant.primary; }
            }
            HorizontalLayout {
                alignment: start;
                pill := ActionBar {
                    floating: true;
                    Button { text: "Undo"; }
                }
            }
            HorizontalLayout {
                alignment: start;
                bulk := BulkActionBar {
                    count: root.selected;
                    clearable: true;
                    clear => {
                        root.clears += 1;
                        root.selected = 0;
                    }
                    Button { text: "Archive"; }
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

fn button(h: &Harness, label: &str) -> Option<Box_> {
    ElementHandle::find_by_accessible_label(h, label)
        .find(|e| e.accessible_role() == Some(AccessibleRole::Button))
        .map(|e| {
            let (at, size) = (e.absolute_position(), e.size());
            Box_ { x: at.x, y: at.y, w: size.width, h: size.height }
        })
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    show(&h);
    h
}

// Docked: its actions at the end, 16 px in, 12 px below a hairline and above the foot; floating:
// a pill 6 px around its actions inside a hairline.
fn docked_or_floating() {
    let h = harness();
    advance(300);
    assert_eq!(h.get_docked_height(), 1.0 + 12.0 + 28.0 + 12.0);
    let save = button(&h, "Save").unwrap();
    assert_eq!(save.x + save.w, 500.0 - 16.0);
    let cancel = button(&h, "Cancel").unwrap();
    assert_eq!(cancel.x + cancel.w + 8.0, save.x);
    assert_eq!(save.y, 13.0);
    assert_eq!(h.get_pill_height(), 2.0 * 7.0 + 28.0);
    let undo = button(&h, "Undo").unwrap();
    assert_eq!(h.get_pill_width(), 2.0 * 7.0 + undo.w);
}

// Each fades in from 8 px lower over BASE, easing out, then stops animating.
fn they_rise_in_then_rest() {
    let h = harness();
    advance(0);
    assert!(h.get_docked_entrance() < 0.01);
    advance(100);
    let mid = h.get_docked_entrance();
    assert!((mid - (1.0 - 0.5f32.powi(3))).abs() < 0.02, "eases out: {mid}");
    assert!(h.window().has_active_animations(), "ticking while it rises");
    advance(120);
    assert_eq!(h.get_docked_entrance(), 1.0);
    assert!(!h.window().has_active_animations(), "nothing keeps ticking once risen");
}

// The bulk bar takes no room with nothing selected; with a selection it rises with the count,
// "selected", a hairline, the actions and the clear button.
fn the_bulk_bar_follows_the_selection() {
    let h = harness();
    advance(300);
    assert_eq!(h.get_bulk_height(), 0.0);
    assert!(button(&h, "Archive").is_none() || button(&h, "Archive").unwrap().h == 0.0);
    h.set_selected(3);
    advance(0);
    assert!(h.get_bulk_entrance() < 0.01, "rises again when it shows");
    advance(300);
    assert_eq!(h.get_bulk_height(), 2.0 * 7.0 + 28.0);
    assert!(ElementHandle::find_by_accessible_label(&h, "selected").count() >= 1);
    let clear = button(&h, "Clear selection").unwrap();
    assert_eq!((clear.w, clear.h), (24.0, 24.0));
    click(h.window(), clear.x + 12.0, clear.y + 12.0);
    assert_eq!(h.get_clears(), 1);
    advance(16);
    assert_eq!(h.get_bulk_height(), 0.0);
}

cases!(action_bar: docked_or_floating, they_rise_in_then_rest, the_bulk_bar_follows_the_selection);
