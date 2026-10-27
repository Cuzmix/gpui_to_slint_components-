#[macro_use]
mod common;
use common::*;

slint::slint! {
    import { SplitPane, SplitCell, ResizeHandle, LayoutMath, MasonrySpot, Theme } from "ui/index.slint";
    export { LayoutMath, MasonrySpot }
    export component Harness inherits Window {
        width: 800px;
        height: 600px;
        in-out property <[float]> sizes: [0.3, 0.7];
        out property <length> left-w: left.width;
        out property <length> right-w: right.width;
        out property <length> right-x: right.x;
        out property <length> top-h: top.height;
        out property <int> reports;
        out property <float> last-share;
        out property <bool> lit: handle.has-hover;

        Rectangle {
            x: 0;
            y: 0;
            width: 606px;
            height: 300px;
            split := SplitPane {
                count: 2;
                sizes: root.sizes;
                min: 96px;
                resized(shares) => {
                    root.reports += 1;
                    root.last-share = shares[0];
                }
                left := SplitCell { share: split.share(0); }
                handle := ResizeHandle {
                    orientation: split.orientation;
                    grabbed => { split.grab(0); }
                    moved(along) => { split.drag(0, along); }
                }
                right := SplitCell {
                    share: split.share(1);
                    inner := SplitPane {
                        orientation: Orientation.vertical;
                        count: 2;
                        top := SplitCell { share: inner.share(0); }
                        ResizeHandle {
                            orientation: inner.orientation;
                            grabbed => { inner.grab(0); }
                            moved(along) => { inner.drag(0, along); }
                        }
                        SplitCell { share: inner.share(1); }
                    }
                }
            }
        }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    ely_slint::wire_layout!(h);
    show(&h);
    advance(16);
    h
}

fn close(a: f32, b: f32) -> bool {
    (a - b).abs() < 0.01
}

// 606 px less the 6 px handle: 600 px shared 0.3 : 0.7. Unset sizes share evenly.
fn panes_take_their_shares() {
    let h = harness();
    assert!(close(h.get_left_w(), 180.0), "{}", h.get_left_w());
    assert!(close(h.get_right_x(), 186.0));
    assert!(close(h.get_right_w(), 420.0));
    assert!(close(h.get_top_h(), (300.0 - 6.0) / 2.0), "{}", h.get_top_h());
}

// The drag counts from its first move, in shares of the whole split (606 px).
fn a_drag_moves_one_divider() {
    let h = harness();
    mouse_move(h.window(), 183.0, 100.0);
    advance(16);
    assert!(h.get_lit());
    mouse_down(h.window(), 183.0, 100.0);
    mouse_move(h.window(), 190.0, 100.0);
    assert!(close(h.get_left_w(), 180.0));
    mouse_move(h.window(), 250.0, 100.0);
    let share = 0.3 + 60.0 / 606.0;
    assert!(close(h.get_last_share(), share));
    // Edges snap to whole pixels, as gpui snaps them.
    assert!((h.get_left_w() - 600.0 * share).abs() <= 1.0, "{}", h.get_left_w());
    // The floor: 96 px of the whole.
    mouse_move(h.window(), -400.0, 100.0);
    assert!(close(h.get_last_share(), 96.0 / 606.0));
    mouse_up(h.window(), -400.0, 100.0);
    assert_eq!(h.get_reports(), 3);
}

fn new_sizes_replace_the_drag() {
    let h = harness();
    mouse_down(h.window(), 183.0, 100.0);
    mouse_move(h.window(), 190.0, 100.0);
    mouse_move(h.window(), 300.0, 100.0);
    mouse_up(h.window(), 300.0, 100.0);
    assert!(h.get_left_w() > 250.0);
    h.set_sizes(slint::ModelRc::new(slint::VecModel::from(vec![1.0, 1.0])));
    advance(16);
    assert!(close(h.get_left_w(), 300.0));
}

fn bad_weights_fall_back_to_even() {
    let h = harness();
    h.set_sizes(slint::ModelRc::new(slint::VecModel::from(vec![-1.0, 2.0])));
    advance(16);
    assert!(close(h.get_left_w(), 300.0));
}

cases!(split: panes_take_their_shares, a_drag_moves_one_divider, new_sizes_replace_the_drag, bad_weights_fall_back_to_even);
