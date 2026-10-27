// SignaturePad, after ely-core/src/forms/signature.rs.
#[macro_use]
mod common;
use common::*;
use i_slint_backend_testing::ElementHandle;
use slint::{ComponentHandle, Model};

slint::slint! {
    import { SignaturePad, PenPoint, Signatures } from "ui/index.slint";
    export { PenPoint, Signatures }
    export component Harness inherits Window {
        width: 500px;
        height: 300px;
        in-out property <[[PenPoint]]> strokes;
        in-out property <int> changes;
        out property <int> drawing: pad.drawing.length;
        pad := SignaturePad {
            x: 10px; y: 10px; width: 420px;
            strokes <=> root.strokes;
            strokes-changed => { root.changes += 1; }
        }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    ely_slint::wire_signatures!(h);
    show(&h);
    h
}

fn points(h: &Harness, ix: usize) -> Vec<(f32, f32)> {
    h.get_strokes().row_data(ix).unwrap().iter().map(|p| (p.x, p.y)).collect()
}

fn a_drag_draws_a_stroke_that_lands_on_lift() {
    let h = harness();
    assert_eq!(ElementHandle::find_by_accessible_label(&h, "Sign here").count(), 1);
    mouse_move(h.window(), 30.0, 30.0);
    mouse_down(h.window(), 30.0, 30.0);
    mouse_move(h.window(), 31.0, 31.0);
    assert_eq!(h.get_drawing(), 1, "a twitch adds nothing");
    mouse_move(h.window(), 60.0, 40.0);
    mouse_move(h.window(), 90.0, 50.0);
    assert_eq!(h.get_drawing(), 3);
    assert_eq!(h.get_changes(), 0, "nothing lands until the pen lifts");
    // Past the pad it holds at the edge, and lifting out there still lands it.
    mouse_move(h.window(), 480.0, 290.0);
    mouse_up(h.window(), 480.0, 290.0);
    assert_eq!(h.get_changes(), 1);
    assert_eq!(points(&h, 0), [(20.0, 20.0), (50.0, 30.0), (80.0, 40.0), (420.0, 160.0)]);
    assert_eq!(h.get_drawing(), 0);
    assert_eq!(ElementHandle::find_by_accessible_label(&h, "Sign here").count(), 0);
}

fn a_tap_leaves_a_dot_and_clear_wipes() {
    let h = harness();
    click(h.window(), 100.0, 100.0);
    assert_eq!(points(&h, 0), [(90.0, 90.0)]);
    click(h.window(), 200.0, 100.0);
    assert_eq!(h.get_strokes().row_count(), 2);
    click_label(&h, "Clear");
    assert_eq!((h.get_strokes().row_count(), h.get_changes()), (0, 3));
    // Empty, Clear is disabled.
    click_label(&h, "Clear");
    assert_eq!(h.get_changes(), 3);
}

cases!(signature: a_drag_draws_a_stroke_that_lands_on_lift, a_tap_leaves_a_dot_and_clear_wipes);
