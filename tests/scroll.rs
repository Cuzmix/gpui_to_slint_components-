#[macro_use]
mod common;
use common::*;
use slint::ComponentHandle;

slint::slint! {
    import { ScrollArea, Scrollbar, ScrollShadow, ScrollToTop, StickyHeader, Theme } from "ui/index.slint";
    component Line inherits Rectangle { height: 30px; }
    export component Harness inherits Window {
        width: 1200px;
        height: 800px;
        in-out property <length> area-y <=> area.content-y;
        out property <float> area-presence: area.presence;
        out property <length> aw: area.width;
        out property <length> ah: area.height;
        out property <length> area-content: area.content-height;
        in-out property <bool> reduced <=> Theme.reduced-motion;

        in-out property <length> bar-y <=> flick.content-y;
        out property <bool> bar-on: bar.on-bar;
        in-out property <bool> bar-auto: false;
        out property <bool> top-shade: shade.top-shaded;
        out property <bool> bottom-shade: shade.bottom-shaded;

        in-out property <length> top-y <=> top-flick.content-y;
        out property <bool> top-shown: to-top.shown;

        in-out property <length> sticky-y <=> sticky-flick.content-y;
        out property <length> pin-a: head-a.pinned;
        out property <length> pin-b: head-b.pinned;
        out property <length> head-a-h: head-a.height;

        // A 320 × 200 well at the origin: 24 lines, 720 px.
        Rectangle {
            x: 0;
            y: 0;
            width: 320px;
            height: 200px;
            area := ScrollArea {
                vertical: true;
                for n in 24: Line { }
            }
        }

        // A bare Flickable wearing a scrollbar and shadows, at x 400.
        Rectangle {
            x: 400px;
            y: 0;
            width: 320px;
            height: 200px;
            flick := Flickable {
                VerticalLayout {
                    for n in 30: Line { }
                }
            }
            shade := ScrollShadow { content-length: flick.content-height; offset: flick.content-y; }
            bar := Scrollbar { content-length: flick.content-height; offset <=> flick.content-y; auto: root.bar-auto; }
        }

        // ScrollToTop over 40 lines, at x 800.
        Rectangle {
            x: 800px;
            y: 0;
            width: 320px;
            height: 200px;
            top-flick := Flickable {
                VerticalLayout {
                    for n in 40: Line { }
                }
            }
            to-top := ScrollToTop { offset <=> top-flick.content-y; }
        }

        // Two sections of 8 lines under pinning headers, at y 400.
        Rectangle {
            x: 0;
            y: 400px;
            width: 320px;
            height: 200px;
            sticky-flick := Flickable {
                VerticalLayout {
                    a := VerticalLayout {
                        head-a := StickyHeader {
                            box-top: sticky-flick.absolute-position.y;
                            section-height: a.height;
                            Rectangle { height: 24px; }
                        }
                        for n in 8: Line { }
                    }
                    b := VerticalLayout {
                        head-b := StickyHeader {
                            box-top: sticky-flick.absolute-position.y;
                            section-height: b.height;
                            Rectangle { height: 24px; }
                        }
                        for n in 8: Line { }
                    }
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

// Hidden until used; shown while hovered.
fn area_bars_show_on_hover() {
    let h = harness();
    assert_eq!(h.get_area_content(), 720.0);
    assert_eq!(h.get_area_presence(), 0.0);
    mouse_move(h.window(), 100.0, 100.0);
    advance(16);
    assert_eq!(h.get_area_presence(), 1.0);
    mouse_move(h.window(), 600.0, 700.0);
    advance(16);
    assert_eq!(h.get_area_presence(), 0.0, "never scrolled: nothing to fade");
}

// Shown while scrolling, idle 900 ms, then a 320 ms linear fade.
fn area_bars_fade_after_scrolling() {
    let h = harness();
    h.set_area_y(-40.0);
    advance(16);
    assert_eq!(h.get_area_presence(), 1.0);
    advance(880);
    assert_eq!(h.get_area_presence(), 1.0);
    // The idle timer fires; reading the value, as a frame does, starts the fade.
    advance(20);
    assert_eq!(h.get_area_presence(), 1.0);
    advance(160);
    assert!((h.get_area_presence() - 0.5).abs() < 0.06, "{}", h.get_area_presence());
    advance(200);
    assert_eq!(h.get_area_presence(), 0.0);
}

fn the_wheel_scrolls_the_area() {
    let h = harness();
    wheel(h.window(), 100.0, 100.0, 0.0, -60.0);
    advance(500);
    assert_eq!(h.get_area_y(), -60.0);
    wheel(h.window(), 100.0, 100.0, 0.0, -2000.0);
    advance(500);
    assert_eq!(h.get_area_y(), -520.0);
}

// span 200, content 900: thumb 44.4 px, travel 155.6 px. The drag counts from the first move.
fn dragging_the_thumb_scrolls() {
    let h = harness();
    let x = 400.0 + 320.0 - 6.0;
    mouse_move(h.window(), x, 10.0);
    mouse_down(h.window(), x, 10.0);
    mouse_move(h.window(), x, 20.0);
    assert_eq!(h.get_bar_y(), 0.0);
    mouse_move(h.window(), x, 20.0 + 77.8);
    let travel = 200.0 - 200.0 * 200.0 / 900.0;
    let expected = -(700.0 * (77.8 / travel));
    assert!((h.get_bar_y() - expected).abs() < 0.01, "{} vs {expected}", h.get_bar_y());
    mouse_move(h.window(), x, 400.0);
    assert_eq!(h.get_bar_y(), -700.0);
    mouse_up(h.window(), x, 400.0);
}

fn the_track_holds_an_auto_bar() {
    let h = harness();
    h.set_bar_auto(true);
    h.set_bar_y(-100.0);
    advance(16);
    mouse_move(h.window(), 400.0 + 320.0 - 3.0, 150.0);
    advance(16);
    assert!(h.get_bar_on());
    advance(3000);
    assert!(h.get_bar_on());
    mouse_move(h.window(), 1000.0, 700.0);
    advance(16);
    assert!(!h.get_bar_on());
}

fn shadows_mark_the_hidden_edges() {
    let h = harness();
    assert!(!h.get_top_shade());
    assert!(h.get_bottom_shade());
    h.set_bar_y(-100.0);
    assert!(h.get_top_shade() && h.get_bottom_shade());
    h.set_bar_y(-700.0);
    assert!(h.get_top_shade() && !h.get_bottom_shade());
}

// Past half a view (100 px) the button shows; a click eases back over 320 ms.
fn scroll_to_top_eases_back() {
    let h = harness();
    h.set_top_y(-99.0);
    assert!(!h.get_top_shown());
    h.set_top_y(-400.0);
    advance(16);
    assert!(h.get_top_shown());
    // The button floats 16 px in from the bottom right.
    let button = i_slint_backend_testing::ElementHandle::find_by_accessible_label(&h, "arrow-up").next().unwrap();
    let at = button.absolute_position();
    let size = button.size();
    assert_eq!((at.x + size.width, at.y + size.height), (800.0 + 320.0 - 16.0, 200.0 - 16.0));
    click(h.window(), at.x + 5.0, at.y + 5.0);
    advance(160);
    let left = 1.0 - (1.0 - 0.125);
    assert!((h.get_top_y() - (-400.0 * left)).abs() < 6.0, "{}", h.get_top_y());
    advance(200);
    assert_eq!(h.get_top_y(), 0.0);
    assert!(!h.get_top_shown());
}

// Section a: 24 + 8 × 30 = 264 px. Its header pins until a's end pushes it.
fn sticky_headers_pin_then_yield() {
    let h = harness();
    assert_eq!(h.get_head_a_h(), 24.0);
    assert_eq!((h.get_pin_a(), h.get_pin_b()), (0.0, 0.0));
    h.set_sticky_y(-100.0);
    advance(16);
    assert_eq!((h.get_pin_a(), h.get_pin_b()), (100.0, 0.0));
    h.set_sticky_y(-250.0);
    advance(16);
    assert_eq!((h.get_pin_a(), h.get_pin_b()), (240.0, 0.0));
    h.set_sticky_y(-300.0);
    advance(16);
    assert_eq!((h.get_pin_a(), h.get_pin_b()), (240.0, 36.0));
}

cases!(scroll: area_bars_show_on_hover, area_bars_fade_after_scrolling, the_wheel_scrolls_the_area, dragging_the_thumb_scrolls, the_track_holds_an_auto_bar, shadows_mark_the_hidden_edges, scroll_to_top_eases_back, sticky_headers_pin_then_yield);
