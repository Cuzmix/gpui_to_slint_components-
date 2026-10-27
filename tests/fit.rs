#[macro_use]
mod common;
use slint::ComponentHandle;

slint::slint! {
    import { TextFormat, PlatformStyle, RollCell, TypeText, LineClamp, Ellipsis, MiddleEllipsis, EllipsisTooltip, TextSize, Theme } from "ui/index.slint";
    export { TextFormat, PlatformStyle, RollCell }
    export component Harness inherits Window {
        width: 600px;
        height: 300px;
        in-out property <string> text: "Good interfaces. Quiet ones";
        in-out property <string> probe: "Good interfaces.…";
        in-out property <length> room: 400px;
        in-out property <string> file: "A very long file name that keeps going.pdf";
        in-out property <string> file-probe: "A very long…going.pdf";
        in-out property <length> file-room: 400px;
        in-out property <length> tip-room: 400px;
        out property <length> probe-w: probe-text.preferred-width;
        out property <length> whole-w: whole.preferred-width;
        out property <length> file-probe-w: file-probe-text.preferred-width;
        out property <string> end-shown: end.shown;
        out property <bool> end-cut: end.cut;
        out property <length> end-h: end.height;
        out property <string> mid-shown: mid.shown;
        out property <bool> tip-cut: tip.cut;
        out property <bool> tip-shown: tip.shown;
        out property <length> tip-h: tip.height;
        probe-text := TypeText { visible: false; text: root.probe; wrap: no-wrap; }
        whole := TypeText { visible: false; text: root.text; wrap: no-wrap; }
        file-probe-text := TypeText { visible: false; text: root.file-probe; wrap: no-wrap; }
        end := Ellipsis { x: 0; y: 0; width: root.room; text: root.text; }
        mid := MiddleEllipsis { x: 0; y: 40px; width: root.file-room; text: root.file; }
        out property <length> row-w: in-row.width;
        in-out property <int> clamp-lines: 2;
        in-out property <string> prose: "Ely draws every component in light and dark, with a high-contrast variant of each palette and a color-blind-safe chart palette.";
        out property <length> clamp-h: clamp.height;

        VerticalLayout {
            x: 0;
            y: 160px;
            width: 200px;
            alignment: start;
            clamp := LineClamp { lines: root.clamp-lines; text: root.prose; }
        }
        HorizontalLayout {
            x: 0;
            y: 120px;
            width: 120px;
            in-row := Ellipsis { text: root.text; }
            Rectangle { width: 50px; }
        }
        tip := EllipsisTooltip { x: 0; y: 80px; width: root.tip-room; text: root.text; }
    }
}

fn wired() -> Harness {
    let h = Harness::new().unwrap();
    ely_slint::wire_text_format!(h);
    common::show(&h);
    h
}

fn settle() {
    common::advance(16);
}

fn a_line_that_fits_stays_whole() {
    let h = wired();
    h.set_room(h.get_whole_w());
    settle();
    assert_eq!((h.get_end_shown().as_str(), h.get_end_cut()), ("Good interfaces. Quiet ones", false));
    // Ambient base size at gpui's phi leading: 21.03 snapped to 21.
    assert_eq!(h.get_end_h(), 21.0);
}

fn an_ellipsis_keeps_the_longest_front_that_fits() {
    let h = wired();
    h.set_room(h.get_probe_w());
    settle();
    assert_eq!(h.get_end_shown().as_str(), "Good interfaces…", "no stop or space before the ellipsis");
    assert!(h.get_end_cut());
}

fn no_room_shows_nothing() {
    let h = wired();
    h.set_room(0.0);
    settle();
    assert_eq!(h.get_end_shown().as_str(), "");
}

fn a_middle_cut_keeps_the_front_and_the_extension() {
    let h = wired();
    h.set_file_room(h.get_file_probe_w());
    settle();
    let cut = h.get_mid_shown();
    assert!(cut.starts_with("A very") && cut.ends_with(".pdf") && cut.contains('…'), "{cut}");
    h.set_file_room(1000.0);
    settle();
    assert_eq!(h.get_mid_shown(), h.get_file());
}

fn a_row_squeezes_the_line_to_its_room() {
    let h = wired();
    assert_eq!(h.get_row_w(), 70.0);
}

fn a_clamp_keeps_whole_lines_and_cuts_the_rest() {
    let h = wired();
    // Base size at phi: 21 px a line.
    assert_eq!(h.get_clamp_h(), 42.0);
    h.set_clamp_lines(1);
    assert_eq!(h.get_clamp_h(), 21.0);
    h.set_clamp_lines(20);
    assert!(h.get_clamp_h() > 42.0 && h.get_clamp_h() < 20.0 * 21.0 && h.get_clamp_h() % 21.0 == 0.0);
    h.set_prose("Short".into());
    h.set_clamp_lines(2);
    assert_eq!(h.get_clamp_h(), 21.0, "fewer lines than the clamp keep their own height");
}

fn unwired_lines_elide_at_the_end() {
    let h = Harness::new().unwrap();
    common::show(&h);
    h.set_room(h.get_probe_w());
    settle();
    assert!(h.get_end_cut());
    assert_eq!(h.get_end_shown(), h.get_text(), "the whole text, elided by Slint");
}

fn the_tooltip_shows_only_while_cut() {
    let h = wired();
    // Base size at 1.4 leading: 18.2 snapped to 18.
    assert_eq!(h.get_tip_h(), 18.0);
    assert!(!h.get_tip_cut());
    common::mouse_move(h.window(), 10.0, 88.0);
    common::mouse_move(h.window(), 12.0, 88.0);
    common::advance(600);
    assert!(!h.get_tip_shown(), "a whole line has no tip");
    common::mouse_move(h.window(), 10.0, 200.0);
    h.set_tip_room(60.0);
    assert!(h.get_tip_cut());
    common::mouse_move(h.window(), 10.0, 88.0);
    common::mouse_move(h.window(), 12.0, 88.0);
    common::advance(499);
    assert!(!h.get_tip_shown());
    common::advance(2);
    assert!(h.get_tip_shown());
}

cases!(fit: a_line_that_fits_stays_whole, an_ellipsis_keeps_the_longest_front_that_fits, no_room_shows_nothing, a_middle_cut_keeps_the_front_and_the_extension, a_row_squeezes_the_line_to_its_room, a_clamp_keeps_whole_lines_and_cuts_the_rest, unwired_lines_elide_at_the_end, the_tooltip_shows_only_while_cut);
