// ColorPicker, after ely-core/src/forms/color/picker.rs and fields.rs.
#[macro_use]
mod common;
use common::*;
use i_slint_backend_testing::ElementHandle;
use slint::platform::Key;
use slint::{Color, ComponentHandle};

slint::slint! {
    import { ColorPicker, ColorFormat, Colors, ColorSample, ParsedColor, Hsva, Choice, Choices, Toggles, OptionLayer, Numbers, ParsedNumber, TextFields, TextFit } from "ui/index.slint";
    export { Colors, ColorSample, ParsedColor, Hsva, Choice, Choices, Toggles, Numbers, ParsedNumber, TextFields, TextFit }
    export component Harness inherits Window {
        width: 600px;
        height: 500px;
        in-out property <color> picked: #3772bb;
        in-out property <bool> alpha: true;
        in-out property <int> changes;
        out property <Hsva> hsva: picker.hsva;
        out property <ColorFormat> format: picker.format;
        out property <length> picker-height: picker.height;
        picker := ColorPicker {
            x: 10px;
            y: 10px;
            width: 260px;
            value <=> root.picked;
            alpha: root.alpha;
            color-changed => { root.changes += 1; }
        }
        other := FocusScope { x: 500px; y: 400px; width: 20px; height: 20px; }
        public function focus-other() { other.focus(); }
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
    show(&h);
    advance(0);
    h
}

// The plane spans x 18..262 and y 18..170; the hue rail y 182..194, the alpha rail y 206..218.
const LEFT: f32 = 18.0;
const WIDE: f32 = 244.0;

fn plane(x: f32, y: f32) -> (f32, f32) {
    (LEFT + x * WIDE, 18.0 + y * 152.0)
}

fn close(a: f32, b: f32) -> bool {
    (a - b).abs() < 0.01
}

fn it_opens_on_its_value() {
    let h = harness();
    let hsva = h.get_hsva();
    assert!(close(hsva.h, 213.18) && close(hsva.s, 0.7059) && close(hsva.v, 0.7333), "{hsva:?}");
    assert_eq!(ElementHandle::find_by_accessible_label(&h, "Color value").next().unwrap().accessible_value().unwrap(), "#3772bb");
}

fn pointing_sets_the_part_under_the_pointer() {
    let h = harness();
    let (x, y) = plane(0.5, 0.2);
    click(h.window(), x, y);
    let hsva = h.get_hsva();
    assert!(close(hsva.s, 0.5) && close(hsva.v, 0.8) && close(hsva.h, 213.18), "{hsva:?}");
    assert_eq!(h.get_changes(), 1);
    let before = h.get_hsva();
    click(h.window(), 400.0, 188.0);
    assert_eq!(h.get_hsva(), before, "a press outside the rail does nothing");
    click(h.window(), LEFT + WIDE - 0.1, 188.0);
    assert!((h.get_hsva().h - 360.0).abs() < 0.2);
    click(h.window(), LEFT + WIDE * 0.25, 212.0);
    assert!(close(h.get_hsva().a, 0.25));
    assert_eq!(h.get_picked().alpha(), 64);
    assert_eq!(h.get_changes(), 3);
}

fn drags_follow_once_past_two_pixels() {
    let h = harness();
    let (x, y) = plane(0.5, 0.5);
    mouse_move(h.window(), x, y);
    mouse_down(h.window(), x, y);
    mouse_move(h.window(), x + 1.0, y + 1.0);
    assert!(close(h.get_hsva().s, 0.5), "a twitch does not drag");
    mouse_move(h.window(), x + WIDE * 0.25, y);
    assert!(close(h.get_hsva().s, 0.75));
    // Past the edge it holds at the edge.
    mouse_move(h.window(), 590.0, 2.0);
    let hsva = h.get_hsva();
    assert!(close(hsva.s, 1.0) && close(hsva.v, 1.0), "{hsva:?}");
    mouse_up(h.window(), 590.0, 2.0);
}

fn grays_keep_their_hue() {
    let h = harness();
    click(h.window(), LEFT + WIDE * 0.5, 188.0);
    assert!(close(h.get_hsva().h, 180.0));
    let (x, y) = plane(0.0, 0.5);
    click(h.window(), x, y);
    let hsva = h.get_hsva();
    assert!(close(hsva.h, 180.0) && hsva.s == 0.0, "{hsva:?}");
    // A gray from the owner keeps the hue too.
    h.set_picked(Color::from_rgb_u8(40, 40, 40));
    advance(0);
    assert!(close(h.get_hsva().h, 180.0));
    // The color it just reported does not reset the hue.
    assert_eq!(h.get_changes(), 2);
}

fn opaque_pickers_hide_alpha() {
    let h = harness();
    h.set_alpha(false);
    h.set_picked(Color::from_argb_u8(0x80, 0x37, 0x72, 0xbb));
    advance(0);
    assert_eq!(h.get_hsva().a, 1.0);
    // The rail is gone: the row moved up into its place.
    assert_eq!(h.get_picker_height(), 8.0 + 152.0 + 12.0 + 12.0 + 12.0 + 28.0 + 12.0 + 28.0);
}

fn hex_reads_on_enter_and_misspellings_put_the_color_back() {
    let h = harness();
    click_label(&h, "Color value");
    let field = ElementHandle::find_by_accessible_label(&h, "Color value").next().unwrap();
    field.set_accessible_value("#0af");
    tap(h.window(), Key::Return);
    advance(0);
    assert_eq!(h.get_picked(), Color::from_rgb_u8(0, 0xaa, 0xff));
    assert_eq!(field.accessible_value().unwrap(), "#0af", "the typed text stays while focused");
    h.invoke_focus_other();
    advance(0);
    assert_eq!(field.accessible_value().unwrap(), "#00aaff");
    click_label(&h, "Color value");
    field.set_accessible_value("#12345");
    h.invoke_focus_other();
    advance(0);
    assert_eq!(field.accessible_value().unwrap(), "#00aaff");
    assert_eq!(h.get_picked(), Color::from_rgb_u8(0, 0xaa, 0xff));
}

// The text field assistive technology names `label` (the scrub grip carries the name too).
fn field(h: &Harness, label: &str) -> ElementHandle {
    ElementHandle::find_by_accessible_label(h, label).find(|e| e.accessible_value().is_some()).unwrap_or_else(|| panic!("a {label} field"))
}

fn click_on(h: &Harness, e: &ElementHandle) {
    let (at, size) = (e.absolute_position(), e.size());
    click(h.window(), at.x + size.width / 2.0, at.y + size.height / 2.0);
}

fn rgb_and_hsv_fields_type_the_color() {
    let h = harness();
    click_label(&h, "Color format");
    advance(0);
    click_label(&h, "RGB");
    advance(0);
    assert_eq!(h.get_format(), ColorFormat::Rgb);
    let red = field(&h, "R");
    assert_eq!(red.accessible_value().unwrap(), "55");
    click_on(&h, &red);
    red.set_accessible_value("255");
    tap(h.window(), Key::Return);
    advance(0);
    assert_eq!(h.get_picked(), Color::from_rgb_u8(255, 0x72, 0xbb));
    click_label(&h, "Color format");
    advance(0);
    click_label(&h, "HSV");
    advance(0);
    let hue = field(&h, "H");
    click_on(&h, &hue);
    hue.set_accessible_value("120");
    tap(h.window(), Key::Return);
    advance(0);
    assert!(close(h.get_hsva().h, 120.0));
    let value = field(&h, "V");
    assert_eq!(value.accessible_value().unwrap(), "100");
}

cases!(color_picker: it_opens_on_its_value, pointing_sets_the_part_under_the_pointer, drags_follow_once_past_two_pixels, grays_keep_their_hue, opaque_pickers_hide_alpha, hex_reads_on_enter_and_misspellings_put_the_color_back, rgb_and_hsv_fields_type_the_color);
