// Rating, after ely-core/src/forms/rating.rs.
#[macro_use]
mod common;
use common::*;
use slint::ComponentHandle;
use slint::platform::Key;

slint::slint! {
    import { Rating } from "ui/index.slint";
    export component Harness inherits Window {
        width: 300px;
        height: 100px;
        in-out property <int> stars: 2;
        in-out property <int> changes;
        in-out property <bool> enabled: true;
        out property <int> hovered: rate.hovered;
        out property <bool> focused: rate.has-focus;
        out property <length> rating-width: rate.width;
        out property <length> rating-height: rate.height;
        public function focus-rate() { rate.focus(); }
        rate := Rating {
            x: 10px;
            y: 10px;
            label: "Review";
            enabled: root.enabled;
            value <=> root.stars;
            value-changed => { root.changes += 1; }
        }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    show(&h);
    h
}

// The middle of star `n` (from 1): 20 px stars, 2 px apart, 3 px in.
fn star_x(n: usize) -> f32 {
    10.0 + 3.0 + (n as f32 - 1.0) * 22.0 + 10.0
}

fn it_measures_five_lg_stars() {
    let h = harness();
    assert_eq!(h.get_rating_width(), 5.0 * 20.0 + 4.0 * 2.0 + 6.0);
    assert_eq!(h.get_rating_height(), 26.0);
}

fn hovering_previews_and_leaving_ends_it() {
    let h = harness();
    mouse_move(h.window(), star_x(4), 23.0);
    assert_eq!(h.get_hovered(), 4);
    mouse_move(h.window(), star_x(1), 23.0);
    assert_eq!(h.get_hovered(), 1);
    mouse_move(h.window(), 250.0, 80.0);
    assert_eq!(h.get_hovered(), 0);
}

fn a_press_rates_and_pressing_the_rating_clears() {
    let h = harness();
    click(h.window(), star_x(4), 23.0);
    assert_eq!(h.get_stars(), 4);
    assert_eq!(h.get_changes(), 1);
    assert!(!h.get_focused(), "a press does not take focus");
    click(h.window(), star_x(4), 23.0);
    assert_eq!(h.get_stars(), 0);
}

fn keys_step_and_jump() {
    let h = harness();
    h.invoke_focus_rate();
    tap(h.window(), Key::RightArrow);
    assert_eq!(h.get_stars(), 3);
    tap(h.window(), Key::DownArrow);
    tap(h.window(), Key::LeftArrow);
    assert_eq!(h.get_stars(), 1);
    tap(h.window(), Key::End);
    assert_eq!(h.get_stars(), 5);
    let changes = h.get_changes();
    tap(h.window(), Key::UpArrow);
    assert_eq!(h.get_changes(), changes, "no change past the top");
    tap(h.window(), Key::Home);
    assert_eq!(h.get_stars(), 0);
}

fn disabled_takes_nothing() {
    let h = harness();
    h.set_enabled(false);
    click(h.window(), star_x(5), 23.0);
    assert_eq!(h.get_stars(), 2);
    assert_eq!(h.get_hovered(), 0);
    tab(h.window());
    assert!(!h.get_focused());
}

cases!(rating: it_measures_five_lg_stars, hovering_previews_and_leaving_ends_it, a_press_rates_and_pressing_the_rating_clears, keys_step_and_jump, disabled_takes_nothing);
