// ProgressBar and ProgressRing, after ely-core/src/motion/progress.rs.
#[macro_use]
mod common;
use common::*;
use i_slint_backend_testing::ElementHandle;
use slint::ComponentHandle;

slint::slint! {
    import { ProgressBar, ProgressRing } from "ui/index.slint";
    export component Harness inherits Window {
        width: 300px;
        height: 200px;
        in property <float> value: 0.2;
        in property <float> buffer: -1;
        out property <float> bar-at: bar.shown;
        out property <float> ring-at: ring.shown;
        out property <length> bar-h: bar.height;
        out property <length> ring-w: ring.width;
        VerticalLayout {
            alignment: start;
            bar := ProgressBar {
                value: root.value;
                buffer: root.buffer;
            }
            HorizontalLayout {
                alignment: start;
                ring := ProgressRing {
                    value: root.value;
                    percent: true;
                }
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

// The first value shows at once; each new one glides there over 200 ms, easing out.
fn a_new_value_glides_from_the_last() {
    let h = harness();
    assert_eq!(h.get_bar_at(), 0.2);
    assert_eq!(h.get_ring_at(), 0.2);
    h.set_value(0.8);
    // Change handlers run when the window next updates; the glide starts there.
    advance(0);
    assert_eq!(h.get_bar_at(), 0.2);
    advance(100);
    let mid = h.get_bar_at();
    // ease-out cubic at half time is 7/8 of the way.
    assert!((mid - (0.2 + 0.6 * 0.875)).abs() < 0.02, "{mid}");
    assert!((h.get_ring_at() - mid).abs() < 0.001);
    advance(150);
    assert_eq!(h.get_bar_at(), 0.8);
}

// The bar is the theme's 2 px; the ring the theme's 48 px, its percent inside.
fn sizes_come_from_the_theme() {
    let h = harness();
    assert_eq!(h.get_bar_h(), 2.0);
    assert_eq!(h.get_ring_w(), 48.0);
    assert_eq!(ElementHandle::find_by_accessible_label(&h, "20%").count(), 0, "the percent is not a second name");
}

// A value past the ends stays at the end (gpui panics, G43).
fn a_value_outside_the_range_is_clamped() {
    let h = harness();
    h.set_value(1.6);
    advance(0);
    advance(400);
    assert_eq!(h.get_bar_at(), 1.0);
    h.set_value(-1.0);
    advance(0);
    advance(400);
    assert_eq!(h.get_bar_at(), 0.0);
}

cases!(progress: a_new_value_glides_from_the_last, sizes_come_from_the_theme, a_value_outside_the_range_is_clamped);
