#[macro_use]
mod common;
use common::*;
use slint::platform::{Key, WindowEvent};
use slint::LogicalPosition;

slint::slint! {
    import { Viewport, FloatingPanel, Theme, PlatformStyle } from "ui/index.slint";
    export component Harness inherits Window {
        width: 800px;
        height: 600px;
        in-out property <PlatformStyle> platform <=> Theme.platform;
        in-out property <length> ox <=> vp.offset-x;
        in-out property <length> oy <=> vp.offset-y;
        in-out property <float> scale <=> vp.scale;
        out property <length> blob-x: blob.x;
        in-out property <length> fx: 720px;
        out property <length> at-x: float.at-x;
        out property <length> at-y: float.at-y;
        out property <int> moves;

        Rectangle {
            x: 0;
            y: 0;
            width: 640px;
            height: 260px;
            vp := Viewport {
                blob := Rectangle { x: vp.apply-x(60px); y: vp.apply-y(50px); width: 140px * vp.scale; height: 84px * vp.scale; }
            }
        }

        float := FloatingPanel {
            title: "Inspector";
            start-x: root.fx;
            start-y: 160px;
            panel-width: 240px;
            moved(x, y) => { root.moves += 1; }
            Rectangle { height: 40px; }
        }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    show(&h);
    advance(16);
    h
}

fn wheel(h: &Harness, x: f32, y: f32, dy: f32) {
    h.window().dispatch_event(WindowEvent::PointerScrolled { position: LogicalPosition::new(x, y), delta_x: 0.0, delta_y: dy });
}

fn a_drag_and_the_wheel_pan() {
    let h = harness();
    mouse_move(h.window(), 300.0, 200.0);
    mouse_down(h.window(), 300.0, 200.0);
    mouse_move(h.window(), 330.0, 190.0);
    mouse_up(h.window(), 330.0, 190.0);
    assert_eq!((h.get_ox(), h.get_oy()), (30.0, -10.0));
    assert_eq!(h.get_blob_x(), 90.0);
    wheel(&h, 300.0, 200.0, -20.0);
    assert_eq!(h.get_oy(), -30.0);
}

// The point under the pointer stays put; the zoom stays in 0.1..8.
fn ctrl_and_the_wheel_zoom_about_the_pointer() {
    let h = harness();
    h.set_platform(PlatformStyle::Linux);
    key_down(h.window(), Key::Control);
    wheel(&h, 100.0, 50.0, -100.0);
    let scale = h.get_scale();
    assert!((scale - std::f32::consts::E).abs() < 1e-3, "{scale}");
    assert!((h.get_ox() - (100.0 - 100.0 * scale)).abs() < 1e-3);
    for _ in 0..10 {
        wheel(&h, 100.0, 50.0, -100.0);
    }
    assert_eq!(h.get_scale(), 8.0);
    key_up(h.window(), Key::Control);
}

// The title bar carries it; at least 32 px stays in the window.
fn a_floating_panel_moves_by_its_bar() {
    let h = harness();
    assert_eq!((h.get_at_x(), h.get_at_y()), (720.0, 160.0));
    mouse_move(h.window(), 730.0, 170.0);
    mouse_down(h.window(), 730.0, 170.0);
    mouse_move(h.window(), 735.0, 170.0);
    mouse_move(h.window(), 635.0, 220.0);
    assert_eq!((h.get_at_x(), h.get_at_y()), (620.0, 210.0));
    mouse_move(h.window(), 2000.0, -300.0);
    mouse_up(h.window(), 2000.0, -300.0);
    assert_eq!((h.get_at_x(), h.get_at_y()), (800.0 - 32.0, 0.0));
    // The anchoring move reports too, as gpui's first drag move does.
    assert_eq!(h.get_moves(), 3);
    h.set_fx(100.0);
    advance(16);
    assert_eq!(h.get_at_x(), 100.0, "a new start replaces the drag");
}

cases!(viewport: a_drag_and_the_wheel_pan, ctrl_and_the_wheel_zoom_about_the_pointer, a_floating_panel_moves_by_its_bar);
