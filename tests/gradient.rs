// GradientEditor, after ely-core/src/forms/color/gradient.rs.
#[macro_use]
mod common;
use common::*;
use i_slint_backend_testing::ElementHandle;
use slint::platform::Key;
use slint::{Color, ComponentHandle, Model};

slint::slint! {
    import { GradientEditor, GradientStop, GradientEdit, Gradients, Colors, ColorSample, ParsedColor, Choice, Choices, Toggles, OptionLayer, Numbers, ParsedNumber, TextFields, TextFit } from "ui/index.slint";
    export { GradientStop, GradientEdit, Gradients, Colors, ColorSample, ParsedColor, Choice, Choices, Toggles, Numbers, ParsedNumber, TextFields, TextFit }
    export component Harness inherits Window {
        width: 600px;
        height: 600px;
        in-out property <[GradientStop]> stops: [{ at: 0, color: #181613 }, { at: 1, color: #e9e7e5 }];
        in-out property <int> changes;
        out property <int> chosen: editor.chosen;
        out property <bool> focused: editor.has-focus;
        editor := GradientEditor {
            x: 10px;
            y: 10px;
            width: 360px;
            label: "Gradient";
            stops <=> root.stops;
            stops-changed => { root.changes += 1; }
        }
        OptionLayer { }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    ely_slint::wire_options!(h);
    ely_slint::wire_toggles!(h);
    ely_slint::wire_fields!(h);
    ely_slint::wire_numbers!(h);
    ely_slint::wire_colors!(h);
    ely_slint::wire_gradients!(h);
    show(&h);
    advance(0);
    h
}

// The bar spans x 18..362 at y 10..38; the stops sit on y 46..62.
const LEFT: f32 = 18.0;
const WIDE: f32 = 344.0;
const BAR: f32 = 24.0;
const RAIL: f32 = 54.0;

fn at(t: f32) -> f32 {
    LEFT + t * WIDE
}

fn places(h: &Harness) -> Vec<f32> {
    h.get_stops().iter().map(|stop| (stop.at * 100.0).round() / 100.0).collect()
}

fn a_press_on_the_bar_adds_a_blended_stop() {
    let h = harness();
    click(h.window(), at(0.25), BAR);
    assert_eq!(places(&h), [0.0, 0.25, 1.0]);
    assert_eq!((h.get_chosen(), h.get_changes()), (1, 1));
    assert_eq!(h.get_stops().row_data(1).unwrap().color, Color::from_rgb_u8(76, 74, 72));
    assert!(!h.get_focused(), "the bar takes no focus");
}

fn dragging_a_stop_moves_it_past_its_neighbors() {
    let h = harness();
    mouse_move(h.window(), at(0.0), RAIL);
    mouse_down(h.window(), at(0.0), RAIL);
    assert!(h.get_focused(), "a press on the stops focuses them");
    mouse_move(h.window(), at(0.0) + 1.0, RAIL);
    assert_eq!(h.get_changes(), 0, "a twitch does not drag");
    mouse_move(h.window(), at(0.5), RAIL);
    assert_eq!(places(&h), [0.5, 1.0]);
    assert_eq!(h.get_chosen(), 0);
    mouse_move(h.window(), 590.0, RAIL);
    mouse_up(h.window(), 590.0, RAIL);
    assert_eq!(places(&h), [1.0, 1.0]);
    assert_eq!(h.get_chosen(), 1, "the dragged stop landed after the other");
    assert_eq!(h.get_stops().row_data(1).unwrap().color, Color::from_rgb_u8(0x18, 0x16, 0x13));
}

fn pressing_a_stop_chooses_it() {
    let h = harness();
    click(h.window(), at(1.0) + 6.0, RAIL);
    assert_eq!((h.get_chosen(), h.get_changes()), (1, 0));
    click(h.window(), 590.0, RAIL);
    assert_eq!(h.get_chosen(), 1, "a press past the handles chooses none");
}

fn keys_nudge_and_delete_the_chosen_stop() {
    let h = harness();
    click(h.window(), at(0.25), BAR);
    click(h.window(), at(0.25), RAIL);
    tap(h.window(), Key::RightArrow);
    assert_eq!(places(&h), [0.0, 0.26, 1.0]);
    let rail = ElementHandle::find_by_accessible_label(&h, "Gradient").next().unwrap();
    assert_eq!(rail.accessible_value().unwrap(), "26");
    assert_eq!(rail.accessible_description().unwrap(), "Stop 2 of 3");
    tap(h.window(), Key::Delete);
    assert_eq!(places(&h), [0.0, 1.0]);
    assert_eq!(h.get_chosen(), 0);
    tap(h.window(), Key::Backspace);
    assert_eq!(places(&h), [0.0, 1.0], "two stops stay");
    tap(h.window(), Key::LeftArrow);
    assert_eq!(places(&h), [0.0, 1.0], "the first stop holds at 0");
}

fn the_picker_recolors_the_chosen_stop() {
    let h = harness();
    click(h.window(), at(1.0), RAIL);
    // The picker's plane: x 18..362, y 78..230; its top left is white.
    click(h.window(), 18.5, 78.5);
    let color = h.get_stops().row_data(1).unwrap().color;
    assert!(color.red() > 250 && color.green() > 250 && color.blue() > 250, "{color:?}");
    assert_eq!(h.get_stops().row_data(0).unwrap().color, Color::from_rgb_u8(0x18, 0x16, 0x13));
    assert_eq!(h.get_changes(), 1);
}

cases!(gradient: a_press_on_the_bar_adds_a_blended_stop, dragging_a_stop_moves_it_past_its_neighbors, pressing_a_stop_chooses_it, keys_nudge_and_delete_the_chosen_stop, the_picker_recolors_the_chosen_stop);
