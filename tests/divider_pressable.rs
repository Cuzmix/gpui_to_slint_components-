#[macro_use]
mod common;
use common::*;

slint::slint! {
    import { Divider, VerticalDivider, Pressable, Theme } from "ui/index.slint";
    export component Harness inherits Window {
        width: 300px;
        height: 200px;
        in-out property <bool> lp-enabled: true;
        in-out property <int> presses;
        in-out property <int> longs;
        out property <bool> down: p.pressed;
        out property <length> plain-h: plain.height;
        out property <length> plain-w: plain.width;
        out property <length> vert-w: vert.width;
        VerticalLayout {
            alignment: start;
            plain := Divider { }
            labeled := Divider { text: "or"; }
            HorizontalLayout {
                height: 50px;
                alignment: start;
                vert := VerticalDivider { }
            }
            p := Pressable {
                height: 50px;
                long-press-enabled: lp-enabled;
                press => { presses += 1; }
                long-press => { longs += 1; }
            }
        }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    show(&h);
    h
}

fn divider_is_a_hairline() {
    let h = harness();
    assert_eq!(h.get_plain_h(), 1.0);
    assert_eq!(h.get_plain_w(), 300.0);
    assert_eq!(h.get_vert_w(), 1.0);
}

fn short_press_fires_press() {
    let h = harness();
    let y = 1.0 + 16.0 + 50.0 + 20.0;
    mouse_move(h.window(), 10.0, y);
    mouse_down(h.window(), 10.0, y);
    assert!(h.get_down());
    advance(499);
    mouse_up(h.window(), 10.0, y);
    assert_eq!((h.get_presses(), h.get_longs()), (1, 0));
    assert!(!h.get_down());
}

fn long_press_fires_once_and_swallows_the_release() {
    let h = harness();
    let y = 1.0 + 16.0 + 50.0 + 20.0;
    mouse_move(h.window(), 10.0, y);
    mouse_down(h.window(), 10.0, y);
    advance(500);
    assert_eq!(h.get_longs(), 1);
    advance(1000);
    assert_eq!(h.get_longs(), 1);
    mouse_up(h.window(), 10.0, y);
    assert_eq!(h.get_presses(), 0);
}

fn without_long_press_a_slow_press_still_presses() {
    let h = harness();
    h.set_lp_enabled(false);
    let y = 1.0 + 16.0 + 50.0 + 20.0;
    mouse_move(h.window(), 10.0, y);
    mouse_down(h.window(), 10.0, y);
    advance(900);
    mouse_up(h.window(), 10.0, y);
    assert_eq!((h.get_presses(), h.get_longs()), (1, 0));
}

fn release_outside_fires_nothing() {
    let h = harness();
    let y = 1.0 + 16.0 + 50.0 + 20.0;
    mouse_move(h.window(), 10.0, y);
    mouse_down(h.window(), 10.0, y);
    mouse_move(h.window(), 10.0, 195.0);
    mouse_up(h.window(), 10.0, 195.0);
    assert_eq!((h.get_presses(), h.get_longs()), (0, 0));
}

cases!(divider_pressable: divider_is_a_hairline, short_press_fires_press, long_press_fires_once_and_swallows_the_release, without_long_press_a_slow_press_still_presses, release_outside_fires_nothing);
