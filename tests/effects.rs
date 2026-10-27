#[macro_use]
mod common;
use common::*;
use i_slint_backend_testing::ElementHandle;

slint::slint! {
    import { TextFormat, PlatformStyle, RollCell, Theme, Typewriter, GradientText, ShimmerText, AnimatedNumber } from "ui/index.slint";
    export { TextFormat, PlatformStyle, RollCell }
    export component Harness inherits Window {
        width: 400px;
        height: 300px;
        in-out property <string> typed: "Hello";
        in-out property <bool> reduced <=> Theme.reduced-motion;
        in-out property <float> number: 999;
        in-out property <bool> count-up: false;
        out property <string> shown: tw.shown-text;
        out property <bool> typing: tw.typing;
        out property <length> tw-h: tw.preferred-height;
        out property <length> grad-h: grad.height;
        out property <length> shimmer-h: shimmer.height;
        out property <length> num-h: num.height;
        out property <string> num-text: num.settled-text;
        out property <bool> num-moving: num.moving;
        tw := Typewriter { x: 0; y: 0; text: root.typed; }
        grad := GradientText { x: 0; y: 40px; width: 200px; text: "Ely"; }
        shimmer := ShimmerText { x: 0; y: 80px; width: 200px; text: "Thinking…"; }
        num := AnimatedNumber { x: 0; y: 120px; value: root.number; count-up: root.count-up; }
    }
}

fn wired() -> Harness {
    let h = Harness::new().unwrap();
    ely_slint::wire_text_format!(h);
    show(&h);
    h
}

fn labelled(h: &Harness, label: &str) -> usize {
    ElementHandle::find_by_accessible_label(h, label).count()
}

fn the_typewriter_reveals_a_grapheme_at_a_time() {
    let h = wired();
    h.set_typed("Héllo👋🏽".into());
    advance(0);
    assert!(h.get_typing());
    assert_eq!(h.get_shown(), "");
    // Six graphemes at 32 a second: 187.5 ms in all.
    advance(40);
    assert_eq!(h.get_shown(), "Hé");
    advance(110);
    assert_eq!(h.get_shown(), "Héllo");
    advance(40);
    assert_eq!(h.get_shown(), "Héllo👋🏽");
    assert!(!h.get_typing());
    // Base text at phi is 21 px; the 16.9 px caret sits inside.
    assert_eq!(h.get_tw_h(), 21.0);
}

fn reduced_motion_types_at_once() {
    let h = wired();
    h.set_reduced(true);
    h.set_typed("Hello, world.".into());
    advance(0);
    advance(2);
    assert_eq!(h.get_shown(), "Hello, world.");
}

fn gradient_and_shimmer_sit_on_the_ambient_line() {
    let h = wired();
    assert_eq!(h.get_grad_h(), 21.0);
    assert_eq!(h.get_shimmer_h(), 21.0);
    h.set_reduced(true);
    assert_eq!(labelled(&h, "Thinking…"), 2, "the shimmer and its still text");
}

fn digits_roll_to_each_new_value() {
    let h = wired();
    // xxl, 24 px at 1.2: 28.8 rounded to 29.
    assert_eq!(h.get_num_h(), 29.0);
    assert_eq!(h.get_num_text(), "999");
    h.set_number(1000.0);
    advance(0);
    assert!(h.get_num_moving());
    assert_eq!(h.get_num_text(), "1,000");
    advance(320);
    assert!(h.get_num_moving());
    // The first place rolls from a blank (0) to 1: at ease-out-cubic(0.5) = 0.875 its column
    // sits 0.875 lines up, so its 1 is 0.125 lines below the row's top.
    let first_one = ElementHandle::find_by_accessible_label(&h, "1")
        .min_by(|a, b| a.absolute_position().x.total_cmp(&b.absolute_position().x))
        .unwrap();
    assert!((first_one.absolute_position().y - (120.0 + 29.0 * 0.125)).abs() < 0.01, "{:?}", first_one.absolute_position());
    advance(330);
    assert!(!h.get_num_moving());
}

fn counting_walks_through_the_values() {
    let h = wired();
    h.set_count_up(true);
    advance(0);
    h.set_number(100.0);
    advance(0);
    assert_eq!(labelled(&h, "999"), 1, "still at the start; the node reads where it lands");
    // ease-out-cubic at a half is 0.875: 999 + (100 - 999) * 0.875.
    advance(320);
    assert_eq!(labelled(&h, "212"), 1);
    advance(330);
    assert_eq!(labelled(&h, "100"), 2);
}

cases!(effects: the_typewriter_reveals_a_grapheme_at_a_time, reduced_motion_types_at_once, gradient_and_shimmer_sit_on_the_ambient_line, digits_roll_to_each_new_value, counting_walks_through_the_values);
