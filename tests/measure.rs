#[macro_use]
mod common;
use common::*;

slint::slint! {
    import { Measure, IntersectionObserver } from "ui/index.slint";
    export component Harness inherits Window {
        width: 400px;
        height: 300px;
        in-out property <length> mw;
        in-out property <length> mx;
        in-out property <length> box-w: 50px;
        in-out property <length> probe-y: 100px;
        in-out property <int> reports;
        in-out property <bool> last-seen;
        out property <bool> in-view: probe.in-view;
        Measure {
            x: 20px;
            y: 100px;
            width: box-w;
            height: 20px;
            measured(x, y, w, h) => { mx = x; mw = w; }
        }
        probe := IntersectionObserver {
            x: 200px;
            y: probe-y;
            width: 20px;
            height: 20px;
            // The visible area: the top 200 px of the window.
            visible-x: 0;
            visible-y: 0;
            visible-width: 400px;
            visible-height: 200px;
            visibility-changed(seen) => { reports += 1; last-seen = seen; }
        }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    show(&h);
    h
}

fn measure_reports_bounds_and_changes() {
    let h = harness();
    advance(1);
    assert_eq!((h.get_mx(), h.get_mw()), (20.0, 50.0));
    h.set_box_w(80.0);
    advance(1);
    assert_eq!(h.get_mw(), 80.0);
}

// GPUI reports once on first paint, then only when visibility flips.
fn observer_reports_entering_and_leaving_once_each() {
    let h = harness();
    advance(1);
    assert_eq!((h.get_reports(), h.get_last_seen()), (1, true));
    h.set_probe_y(150.0);
    advance(1);
    assert_eq!(h.get_reports(), 1);
    h.set_probe_y(250.0);
    advance(1);
    assert_eq!((h.get_reports(), h.get_last_seen(), h.get_in_view()), (2, false, false));
    h.set_probe_y(190.0);
    advance(1);
    assert_eq!((h.get_reports(), h.get_last_seen()), (3, true));
}

cases!(measure: measure_reports_bounds_and_changes, observer_reports_entering_and_leaving_once_each);
