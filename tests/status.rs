// OfflineIndicator and ZoomControl, after ely-core/src/shell/status.rs.
#[macro_use]
mod common;
use std::cell::RefCell;
use std::rc::Rc;

use common::*;
use i_slint_backend_testing::ElementHandle;
use slint::ComponentHandle;

slint::slint! {
    import { OfflineIndicator, ZoomControl, ZoomStep, ZoomSteps, Connectivity, WindowChrome, TextFormat, PlatformStyle, RollCell } from "ui/index.slint";
    export { Connectivity, ZoomStep, WindowChrome, TextFormat, PlatformStyle, RollCell }
    export component Harness inherits Window {
        width: 500px;
        height: 200px;
        in property <Connectivity> state: Connectivity.online;
        in property <int> owned-percent: 125;
        out property <bool> showing: pill.showing;
        out property <length> pill-h: pill.height;
        callback stepped(ZoomStep);
        callback seen(Connectivity, Connectivity);
        pure function up(c: int) -> int { ZoomSteps.up(c) }
        pure function down(c: int) -> int { ZoomSteps.down(c) }
        out property <[int]> walk: [up(100), down(100), down(103), up(200), down(50)];

        VerticalLayout {
            alignment: start;
            spacing: 10px;
            pill := OfflineIndicator {
                state: root.state;
                changed-state(from, to) => { root.seen(from, to); }
            }
            HorizontalLayout {
                alignment: start;
                ZoomControl { }
            }
            HorizontalLayout {
                alignment: start;
                ZoomControl {
                    owned: true;
                    percent: root.owned-percent;
                    zoom(step) => { root.stepped(step); }
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

fn find(h: &Harness, label: &str) -> Vec<Box_> {
    ElementHandle::find_by_accessible_label(h, label)
        .map(|e| {
            let (at, size) = (e.absolute_position(), e.size());
            Box_ { x: at.x, y: at.y, w: size.width, h: size.height }
        })
        .collect()
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    ely_slint::wire_text_format!(h);
    show(&h);
    advance(16);
    h
}

fn press(h: &Harness, b: &Box_) {
    click(h.window(), b.x + b.w / 2.0, b.y + b.h / 2.0);
}

// gpui's step test: up and down the scale, stopping at its ends.
fn steps_walk_the_scale_and_stop_at_its_ends() {
    let h = harness();
    let walk: Vec<i32> = slint::Model::iter(&h.get_walk()).collect();
    assert_eq!(walk, vec![110, 90, 100, -1, -1]);
}

// Online it takes no room; offline and reconnecting it shows; back online it stays 2.4 s.
fn the_pill_shows_while_away_and_briefly_once_back() {
    let h = harness();
    let seen = Rc::new(RefCell::new(Vec::new()));
    let log = seen.clone();
    h.on_seen(move |from, to| log.borrow_mut().push((from, to)));
    assert!(!h.get_showing());
    assert_eq!(h.get_pill_h(), 0.0);
    h.set_state(Connectivity::Offline);
    advance(0);
    advance(250);
    assert!(h.get_showing());
    assert!(!find(&h, "Offline · changes stay on this device").is_empty());
    h.set_state(Connectivity::Reconnecting);
    advance(16);
    assert!(!find(&h, "Reconnecting…").is_empty());
    h.set_state(Connectivity::Online);
    advance(0);
    advance(2000);
    assert!(h.get_showing());
    assert!(!find(&h, "Back online").is_empty());
    advance(500);
    assert!(!h.get_showing());
    assert_eq!(
        *seen.borrow(),
        vec![
            (Connectivity::Online, Connectivity::Offline),
            (Connectivity::Offline, Connectivity::Reconnecting),
            (Connectivity::Reconnecting, Connectivity::Online)
        ]
    );
}

// The window's control steps the window's zoom and resets it; at the top, plus is off.
fn the_window_control_steps_the_window_zoom() {
    let h = harness();
    let asked = Rc::new(RefCell::new(Vec::new()));
    let log = asked.clone();
    let chrome = h.global::<WindowChrome>();
    chrome.on_set_zoom(move |p| log.borrow_mut().push(p));
    let zoom_in = find(&h, "zoom-in");
    let zoom_out = find(&h, "zoom-out");
    press(&h, &zoom_in[0]);
    press(&h, &zoom_out[0]);
    let reset = find(&h, "100%");
    assert_eq!(reset[0].w, 48.0);
    assert_eq!(reset[0].h, 24.0);
    press(&h, &reset[0]);
    assert_eq!(*asked.borrow(), vec![110, 90, 100]);
    chrome.set_zoom_percent(200);
    advance(16);
    press(&h, &find(&h, "zoom-in")[0]);
    assert_eq!(asked.borrow().len(), 3, "no step above 200");
}

// An owned control shows the owner's percent and hands Out, In and Reset to it.
fn an_owned_control_hands_its_steps_to_the_owner() {
    let h = harness();
    let steps = Rc::new(RefCell::new(Vec::new()));
    let log = steps.clone();
    h.on_stepped(move |s| log.borrow_mut().push(s));
    let reset = find(&h, "125%");
    assert_eq!(reset.len(), 1);
    let zoom_out = find(&h, "zoom-out");
    let zoom_in = find(&h, "zoom-in");
    press(&h, &zoom_out[1]);
    press(&h, &reset[0]);
    press(&h, &zoom_in[1]);
    assert_eq!(*steps.borrow(), vec![ZoomStep::Out, ZoomStep::Reset, ZoomStep::In]);
}

cases!(status: steps_walk_the_scale_and_stop_at_its_ends, the_pill_shows_while_away_and_briefly_once_back, the_window_control_steps_the_window_zoom, an_owned_control_hands_its_steps_to_the_owner);
