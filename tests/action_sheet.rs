// ActionSheet, after ely-core/src/buttons/sheet.rs.
#[macro_use]
mod common;
use common::*;
use i_slint_backend_testing::{AccessibleRole, ElementHandle};
use slint::ComponentHandle;
use slint::platform::Key;

slint::slint! {
    import { ActionSheet, SheetAction, FocusSteps } from "ui/index.slint";
    export { FocusSteps }
    export component Harness inherits Window {
        width: 600px;
        height: 500px;
        in-out property <bool> open: true;
        in-out property <string> heading: "Change photo";
        in-out property <int> picked: -1;
        in-out property <int> closes;
        if root.open: ActionSheet {
            title: root.heading;
            message: "Pick a new one or remove it.";
            actions: [{ label: "Take photo" }, { label: "Remove photo", destructive: true }];
            activated(ix) => { root.picked = ix; }
            close => {
                root.closes += 1;
                root.open = false;
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
    let e = ElementHandle::find_by_accessible_label(h, label)
        .find(|e| e.accessible_role() == Some(AccessibleRole::Button))
        .unwrap_or_else(|| panic!("a button named {label}"));
    let (at, size) = (e.absolute_position(), e.size());
    Box_ { x: at.x, y: at.y, w: size.width, h: size.height }
}

fn press(h: &Harness, label: &str) {
    let b = button(h, label);
    click(h.window(), b.x + b.w / 2.0, b.y + b.h / 2.0);
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    let weak = h.as_weak();
    h.global::<FocusSteps>().on_step(move |forward| {
        if let Some(h) = weak.upgrade() {
            ely_slint::focus::step(h.window(), forward);
        }
    });
    show(&h);
    advance(300);
    h
}

// 380 px wide, centered, 16 px off the bottom: the choices' card, 8 px, then Cancel's. Rows are a
// lg control and a half tall. It is named by its title and described by its message.
fn it_rises_from_the_bottom() {
    let h = harness();
    let (take, remove, cancel) = (button(&h, "Take photo"), button(&h, "Remove photo"), button(&h, "Cancel"));
    assert_eq!((cancel.x, cancel.w), ((600.0 - 380.0) / 2.0 + 1.0, 380.0 - 2.0));
    assert_eq!(cancel.y + cancel.h, 500.0 - 16.0 - 1.0);
    assert_eq!((take.h, remove.h, cancel.h), (44.0, 44.0, 44.0));
    assert_eq!(remove.y, take.y + take.h);
    assert_eq!(cancel.y, remove.y + remove.h + 1.0 + 8.0 + 1.0);
    let sheet = ElementHandle::find_by_accessible_label(&h, "Change photo")
        .find(|e| e.accessible_role() == Some(AccessibleRole::Groupbox))
        .expect("the sheet");
    assert_eq!(sheet.accessible_description().as_deref(), Some("Pick a new one or remove it."));
}

// It rises 24 px into place over BASE.
fn it_rises_in() {
    let h = Harness::new().unwrap();
    show(&h);
    advance(0);
    let start = button(&h, "Cancel").y;
    advance(300);
    assert_eq!(start - button(&h, "Cancel").y, 24.0);
}

// A choice reports itself, then closes; Cancel, Escape and the scrim close.
fn choices_and_ways_out() {
    let h = harness();
    press(&h, "Remove photo");
    assert_eq!((h.get_picked(), h.get_closes()), (1, 1));
    h.set_open(true);
    advance(300);
    press(&h, "Cancel");
    assert_eq!(h.get_closes(), 2);
    h.set_open(true);
    advance(300);
    tap(h.window(), Key::Escape);
    assert_eq!(h.get_closes(), 3);
    h.set_open(true);
    advance(300);
    click(h.window(), 300.0, 20.0);
    assert_eq!(h.get_closes(), 4);
    assert_eq!(h.get_picked(), 1, "only the choice reports");
}

// It takes focus; Tab reaches the rows, wraps inside the sheet, and Enter chooses.
fn keys_choose() {
    let h = harness();
    for _ in 0..4 {
        tab(h.window());
    }
    tap(h.window(), Key::Return);
    assert_eq!(h.get_picked(), 0);
    assert_eq!(h.get_closes(), 1);
}

// Named "Actions" without a title or message.
fn untitled_it_is_named_actions() {
    let h = harness();
    h.set_heading("".into());
    advance(16);
    assert!(ElementHandle::find_by_accessible_label(&h, "Pick a new one or remove it.")
        .any(|e| e.accessible_role() == Some(AccessibleRole::Groupbox)));
}

cases!(action_sheet: it_rises_from_the_bottom, it_rises_in, choices_and_ways_out, keys_choose, untitled_it_is_named_actions);
