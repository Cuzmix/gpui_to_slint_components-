// Color math, ColorSwatch, ColorPalette and EyeDropper, after ely-core/src/forms/color/mod.rs,
// swatch.rs and dropper.rs.
#[macro_use]
mod common;
use common::*;
use slint::{Color, ComponentHandle, SharedString};
use slint::platform::Key;

slint::slint! {
    import { ControlSize, ColorSwatch, ColorPalette, NamedColor, EyeDropper, Colors, ColorMath, ColorSample, ParsedColor, Hsva } from "ui/index.slint";
    export { Colors, ColorSample, ParsedColor, Hsva }
    export component Harness inherits Window {
        width: 400px;
        height: 200px;
        in-out property <color> chosen-color;
        in-out property <bool> chosen;
        in-out property <int> picks;
        in-out property <color> dropped;
        in-out property <int> drops;
        in-out property <int> swatch-clicks;
        out property <bool> dropper-enabled: dropper.available;
        out property <bool> swatch-focused: plain.has-focus;
        out property <length> palette-height: palette.height;
        public pure function hex(c: color) -> string { ColorMath.hex(c) }
        public pure function color-of(h: Hsva) -> color { ColorMath.color-of(h) }
        public pure function hsva-of(c: color, hue: float) -> Hsva { ColorMath.hsva-of(c, hue) }
        plain := ColorSwatch {
            x: 10px; y: 10px;
            color: #3772bb66;
            size: ControlSize.md;
            clicked => { root.swatch-clicks += 1; }
        }
        first := ColorSwatch {
            x: 50px; y: 10px;
            color: #181613;
            tooltip: "Ink";
            clickable: true;
            clicked => { root.swatch-clicks += 1; }
        }
        palette := ColorPalette {
            x: 10px; y: 50px;
            width: 200px;
            columns: 4;
            colors: [
                { name: "Red", color: #ef4444 },
                { name: "Blue", color: #3b82f6 },
                { name: "Green", color: #22c55e },
                { name: "Gold", color: #eab308 },
                { name: "Rose", color: #f43f5e },
            ];
            selected <=> root.chosen-color;
            chosen <=> root.chosen;
            selection-changed => { root.picks += 1; }
        }
        dropper := EyeDropper {
            x: 300px; y: 10px;
            owner: "drop";
            picked(c) => { root.dropped = c; root.drops += 1; }
        }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    ely_slint::wire_colors!(h);
    show(&h);
    h
}

fn hsva(h: f32, s: f32, v: f32, a: f32) -> Hsva {
    Hsva { h, s, v, a }
}

fn hsv_round_trips_and_grays_keep_their_hue() {
    let h = harness();
    let orange = h.invoke_color_of(hsva(30.0, 1.0, 1.0, 1.0));
    assert_eq!(h.invoke_hex(orange).as_str(), "#ff8000");
    let back = h.invoke_hsva_of(orange, 0.0);
    assert!((back.h - 30.1).abs() < 0.2 && (back.s - 1.0).abs() < 1e-4, "{back:?}");
    let gray = h.invoke_hsva_of(Color::from_rgb_u8(128, 128, 128), 210.0);
    assert_eq!((gray.h, gray.s), (210.0, 0.0));
    // Every sector, and a hue past a turn.
    for (degrees, hex) in [(0.0, "#ff0000"), (60.0, "#ffff00"), (120.0, "#00ff00"), (180.0, "#00ffff"), (240.0, "#0000ff"), (300.0, "#ff00ff"), (360.0, "#ff0000"), (-60.0, "#ff00ff")] {
        assert_eq!(h.invoke_hex(h.invoke_color_of(hsva(degrees, 1.0, 1.0, 1.0))).as_str(), hex, "{degrees}");
    }
    assert_eq!(h.invoke_hex(h.invoke_color_of(hsva(210.0, 0.5, 0.8, 0.5))).as_str(), "#6699cc80");
}

fn hex_spells_alpha_only_when_see_through() {
    let h = harness();
    assert_eq!(h.invoke_hex(Color::from_argb_u8(255, 0x37, 0x72, 0xbb)).as_str(), "#3772bb");
    assert_eq!(h.invoke_hex(Color::from_argb_u8(0x80, 0x37, 0x72, 0xbb)).as_str(), "#3772bb80");
    let parse = |text: &str| h.global::<Colors>().invoke_parse_hex(text.into());
    assert_eq!(parse("#0af").color, Color::from_rgb_u8(0, 0xaa, 0xff));
    assert!(!parse("#12345").ok);
}

fn a_plain_swatch_is_no_button() {
    let h = harness();
    click(h.window(), 24.0, 24.0);
    tab(h.window());
    assert_eq!(h.get_swatch_clicks(), 0);
    assert!(!h.get_swatch_focused());
    let named = i_slint_backend_testing::ElementHandle::find_by_accessible_label(&h, "Ink").count();
    assert_eq!(named, 1, "the clickable swatch is named by its tooltip");
}

fn a_clickable_swatch_clicks_without_taking_focus() {
    let h = harness();
    click(h.window(), 62.0, 22.0);
    assert_eq!(h.get_swatch_clicks(), 1);
    // The first Tab stop: the plain swatch is none.
    tab(h.window());
    tap(h.window(), Key::Return);
    assert_eq!(h.get_swatch_clicks(), 2);
}

fn the_palette_wraps_and_picks() {
    let h = harness();
    // Four 24 px swatches 6 px apart a row; five wrap to two rows.
    assert_eq!(h.get_palette_height(), 54.0);
    click_label(&h, "Blue");
    assert_eq!((h.get_picks(), h.get_chosen()), (1, true));
    assert_eq!(h.get_chosen_color(), Color::from_rgb_u8(0x3b, 0x82, 0xf6));
    let blue = i_slint_backend_testing::ElementHandle::find_by_accessible_label(&h, "Blue").next().unwrap();
    assert_eq!(blue.accessible_item_selected(), Some(true));
    let rose = i_slint_backend_testing::ElementHandle::find_by_accessible_label(&h, "Rose").next().unwrap();
    assert_eq!(rose.absolute_position().y, 50.0 + 30.0, "the fifth swatch opens the second row");
    // Tab reaches each swatch; Space picks.
    tab(h.window());
    tab(h.window());
    tap(h.window(), " ");
    assert_eq!(h.get_chosen_color(), Color::from_rgb_u8(0xef, 0x44, 0x44));
}

fn the_dropper_waits_for_the_host() {
    let h = harness();
    assert!(!h.get_dropper_enabled());
    let asked = std::rc::Rc::new(std::cell::RefCell::new(Vec::<SharedString>::new()));
    let seen = asked.clone();
    h.global::<Colors>().on_sample(move |owner| seen.borrow_mut().push(owner));
    h.global::<Colors>().set_can_sample(true);
    assert!(h.get_dropper_enabled());
    click(h.window(), 314.0, 24.0);
    assert_eq!(asked.borrow().as_slice(), ["drop"]);
    ely_slint::deliver_sample!(h, "elsewhere", Color::from_rgb_u8(1, 2, 3));
    advance(0);
    assert_eq!(h.get_drops(), 0);
    ely_slint::deliver_sample!(h, "drop", Color::from_rgb_u8(55, 114, 187));
    advance(0);
    assert_eq!((h.get_drops(), h.get_dropped()), (1, Color::from_rgb_u8(55, 114, 187)));
}

cases!(color: hsv_round_trips_and_grays_keep_their_hue, hex_spells_alpha_only_when_see_through, a_plain_swatch_is_no_button, a_clickable_swatch_clicks_without_taking_focus, the_palette_wraps_and_picks, the_dropper_waits_for_the_host);
