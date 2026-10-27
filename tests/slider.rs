// Slider and RangeSlider, after ely-core/src/forms/slider.rs.
#[macro_use]
mod common;
use common::*;
use slint::ComponentHandle;
use slint::platform::Key;

slint::slint! {
    import { Slider, RangeSlider, SliderMath } from "ui/index.slint";
    export component Harness inherits Window {
        width: 300px;
        height: 300px;
        in-out property <float> volume: 40;
        in-out property <int> changes;
        in-out property <float> low: 20;
        in-out property <float> high: 80;
        in-out property <int> spans;
        in-out property <bool> enabled: true;
        in-out property <float> tall-value: 50;
        out property <bool> one-focused: one.has-focus;
        out property <bool> span-focused: span.has-focus;
        out property <length> one-height: one.height;
        out property <float> snapped: SliderMath.value-at(0.26, 0, 100, 5);
        out property <float> clamped: SliderMath.value-at(1.4, 0, 100, 5);
        out property <float> centered: SliderMath.value-at(0.5, -1, 1, 0.5);
        public function focus-one() { one.focus(); }
        VerticalLayout {
            x: 10px;
            y: 10px;
            width: 216px;
            height: 280px;
            spacing: 20px;
            alignment: start;
            one := Slider {
                label: "Volume";
                enabled: root.enabled;
                value <=> root.volume;
                value-changed => { root.changes += 1; }
            }
            span := RangeSlider {
                label: "Price";
                step: 5;
                low <=> root.low;
                high <=> root.high;
                range-changed => { root.spans += 1; }
            }
            HorizontalLayout {
                alignment: start;
                height: 116px;
                tall := Slider {
                    vertical: true;
                    value <=> root.tall-value;
                }
            }
        }
    }
}

// The track runs from x 18 to 218 (8 px in at each end of the 216 px slider at x 10).
fn track_x(fraction: f32) -> f32 {
    18.0 + fraction * 200.0
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    show(&h);
    h
}

fn values_snap_and_clamp() {
    let h = harness();
    assert_eq!(h.get_snapped(), 25.0);
    assert_eq!(h.get_clamped(), 100.0);
    assert_eq!(h.get_centered(), 0.0);
    assert_eq!(h.get_one_height(), 16.0);
}

fn a_track_press_jumps_and_focuses() {
    let h = harness();
    click(h.window(), track_x(0.75), 18.0);
    assert_eq!(h.get_volume(), 75.0);
    assert_eq!(h.get_changes(), 1);
    assert!(h.get_one_focused());
    click(h.window(), track_x(0.75), 18.0);
    assert_eq!(h.get_changes(), 1, "no change, no report");
}

fn dragging_moves_and_clamps() {
    let h = harness();
    // Grab the thumb at 40.
    mouse_down(h.window(), track_x(0.4), 18.0);
    mouse_move(h.window(), track_x(0.6), 18.0);
    assert_eq!(h.get_volume(), 60.0);
    mouse_move(h.window(), 290.0, 100.0);
    assert_eq!(h.get_volume(), 100.0);
    mouse_up(h.window(), 290.0, 100.0);
    mouse_move(h.window(), track_x(0.1), 18.0);
    assert_eq!(h.get_volume(), 100.0, "the drag ended");
}

fn keys_step_jump_and_reach_the_ends() {
    let h = harness();
    h.invoke_focus_one();
    tap(h.window(), Key::RightArrow);
    assert_eq!(h.get_volume(), 41.0);
    tap(h.window(), Key::PageDown);
    assert_eq!(h.get_volume(), 31.0);
    tap(h.window(), Key::End);
    assert_eq!(h.get_volume(), 100.0);
    tap(h.window(), Key::Home);
    tap(h.window(), Key::DownArrow);
    assert_eq!(h.get_volume(), 0.0);
}

fn a_range_moves_the_nearer_thumb_and_never_crosses() {
    let h = harness();
    // The range slider sits 36 px down: its middle at y 54.
    click(h.window(), track_x(0.3), 54.0);
    assert_eq!((h.get_low(), h.get_high()), (30.0, 80.0));
    click(h.window(), track_x(0.7), 54.0);
    assert_eq!((h.get_low(), h.get_high()), (30.0, 70.0));
    assert!(h.get_span_focused());
    // Drag the high thumb below the low one: it stops there.
    mouse_down(h.window(), track_x(0.7), 54.0);
    mouse_move(h.window(), track_x(0.1), 54.0);
    mouse_up(h.window(), track_x(0.1), 54.0);
    assert_eq!((h.get_low(), h.get_high()), (30.0, 30.0));
    tap(h.window(), Key::PageUp);
    assert_eq!((h.get_low(), h.get_high()), (30.0, 80.0));
    shift_tab(h.window());
    tap(h.window(), Key::End);
    assert_eq!((h.get_low(), h.get_high()), (80.0, 80.0), "the low thumb stops at the high one");
    assert_eq!(h.get_spans(), 5);
}

fn a_vertical_track_runs_bottom_to_top() {
    let h = harness();
    // The vertical slider sits at y 82, 116 tall: its track from y 90 (top) to 190 (bottom).
    click(h.window(), 18.0, 165.0);
    assert_eq!(h.get_tall_value(), 25.0);
    assert_eq!(h.get_volume(), 40.0);
}

fn disabled_takes_no_press() {
    let h = harness();
    h.set_enabled(false);
    click(h.window(), track_x(0.9), 18.0);
    assert_eq!(h.get_volume(), 40.0);
    assert!(!h.get_one_focused());
}

cases!(slider: values_snap_and_clamp, a_track_press_jumps_and_focuses, dragging_moves_and_clamps, keys_step_jump_and_reach_the_ends, a_range_moves_the_nearer_thumb_and_never_crosses, a_vertical_track_runs_bottom_to_top, disabled_takes_no_press);
