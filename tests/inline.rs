#[macro_use]
mod common;
use i_slint_backend_testing::ElementHandle;
use slint::ComponentHandle;

slint::slint! {
    import { Theme, PlatformStyle, TextFormat, RollCell, TypeText, Code, Blockquote, Kbd, KbdCombo, Highlight, TextSize } from "ui/index.slint";
    export { TextFormat, PlatformStyle, RollCell }
    export component Harness inherits Window {
        width: 600px;
        height: 400px;
        in-out property <PlatformStyle> platform <=> Theme.platform;
        in-out property <string> keys: "secondary-s";
        in-out property <string> chords: "cmd-k cmd-s";
        in-out property <string> query: "report";
        out property <length> code-w: code.width;
        out property <length> code-h: code.height;
        out property <length> code-text-w: code-probe.preferred-width;
        out property <bool> code-fill: code.background == Theme.colors.sunken && code.border-color == Theme.colors.border;
        out property <length> quote-h: quote.height;
        out property <length> kbd-h: kbd.preferred-height;
        out property <length> kbd-box-w: kbd.width;
        out property <length> kbd-w: kbd.preferred-width;
        out property <length> hl-w: hl.preferred-width;
        out property <length> hl-whole-w: hl-probe.preferred-width;
        out property <int> hl-runs: hl.runs.length;
        code := Code { x: 0; y: 0; text: "cargo build"; }
        code-probe := TypeText { visible: false; text: "cargo build"; font-family: Theme.mono-family; line-ratio: Theme.mono-line-ratio; font-size: Theme.text-size(TextSize.sm); }
        quote := Blockquote { x: 0; y: 40px; width: 300px; text: "Simplicity is prerequisite for reliability."; }
        HorizontalLayout {
            x: 0;
            y: 120px;
            height: 30px;
            alignment: start;
            kbd := Kbd { source: root.keys; }
        }
        combo := KbdCombo { x: 0; y: 160px; source: root.chords; }
        hl := Highlight { x: 0; y: 200px; text: "Quarterly report.pdf"; query: root.query; }
        hl-probe := TypeText { visible: false; text: "Quarterly report.pdf"; wrap: no-wrap; }
    }
}

fn wired() -> Harness {
    let h = Harness::new().unwrap();
    ely_slint::wire_text_format!(h);
    common::show(&h);
    h
}

// Caps (and Highlight rows) carrying `label`; their inner texts are hidden from accessibility.
fn labels(h: &Harness, label: &str) -> usize {
    ElementHandle::find_by_accessible_label(h, label)
        .filter(|e| e.accessible_role() == Some(i_slint_backend_testing::AccessibleRole::Text))
        .count()
}

fn code_pads_four_across_inside_a_hairline() {
    let h = wired();
    assert!(h.get_code_fill());
    // JetBrains Mono 12 px at phi leading: 19.4 snapped to 19, plus the border.
    assert_eq!(h.get_code_h(), 21.0);
    assert!((h.get_code_w() - (h.get_code_text_w() + 10.0)).abs() < 0.01);
}

fn blockquote_sets_the_passage_behind_a_rule() {
    let h = wired();
    let rule = ElementHandle::find_by_element_type_name(&h, "Rectangle")
        .find(|e| e.size().width == 2.0)
        .expect("the rule");
    assert_eq!(rule.absolute_position().x, 0.0);
    assert_eq!(rule.size().height, h.get_quote_h());
    let passage = ElementHandle::find_by_accessible_label(&h, "Simplicity is prerequisite for reliability.").next().unwrap();
    assert_eq!(passage.absolute_position().x, 18.0);
    // 14 px at phi: 22.65 snapped to 23, on two lines at 282 px wide or one.
    assert_eq!(h.get_quote_h() % 23.0, 0.0);
}

fn kbd_labels_for_the_platform() {
    let h = wired();
    h.set_platform(PlatformStyle::Mac);
    // Kbd reads ⌘ S; the combo beside it reads ⌘ K ⌘ S.
    assert_eq!((labels(&h, "⌘"), labels(&h, "S")), (3, 2));
    // 11 px at phi: 17.8 snapped to 18, a 1 px top and a 2 px lip.
    assert_eq!(h.get_kbd_h(), 21.0);
    h.set_platform(PlatformStyle::Windows);
    assert_eq!((labels(&h, "⌘"), labels(&h, "Ctrl"), labels(&h, "S")), (0, 1, 2));
}

fn a_cap_is_never_narrower_than_its_minimum() {
    let h = wired();
    h.set_platform(PlatformStyle::Mac);
    h.set_keys("s".into());
    h.set_keys("s".into());
    // Slint lays out new repeater rows on the next frame.
    let _ = h.get_kbd_box_w();
    common::advance(16);
    assert!((h.get_kbd_box_w() - 11.0 * 1.8).abs() < 0.01, "{}", h.get_kbd_box_w());
}

fn kbd_combo_joins_caps_with_plus_off_the_mac() {
    let h = wired();
    h.set_platform(PlatformStyle::Linux);
    h.set_keys("k".into());
    let plus = |h: &Harness| ElementHandle::find_by_accessible_label(h, "+").count();
    assert_eq!((labels(&h, "Super"), labels(&h, "K"), plus(&h)), (2, 2, 2));
    h.set_platform(PlatformStyle::Mac);
    assert_eq!((labels(&h, "⌘"), plus(&h)), (2, 0));
}

fn unwired_keys_show_their_source() {
    let h = Harness::new().unwrap();
    common::show(&h);
    assert_eq!(labels(&h, "secondary-s"), 1);
    assert_eq!(labels(&h, "cmd-k cmd-s"), 1);
    assert_eq!(h.get_hl_runs(), 0);
}

fn highlight_marks_every_match() {
    let h = wired();
    assert_eq!(h.get_hl_runs(), 3);
    let marks: Vec<_> = ElementHandle::find_by_element_type_name(&h, "Rectangle")
        .filter(|e| e.absolute_position().y == 200.0)
        .collect();
    assert_eq!(marks.len(), 3);
    // The runs sit edge to edge, as wide as the text set whole.
    // Slint rounds each run up to a whole pixel, so three runs can come out up to 3 px wider.
    let extra = h.get_hl_w() - h.get_hl_whole_w();
    assert!((0.0..3.0).contains(&extra), "{} vs {}", h.get_hl_w(), h.get_hl_whole_w());
    h.set_query("".into());
    assert_eq!(h.get_hl_runs(), 1);
}

cases!(inline: code_pads_four_across_inside_a_hairline, blockquote_sets_the_passage_behind_a_rule, kbd_labels_for_the_platform, a_cap_is_never_narrower_than_its_minimum, kbd_combo_joins_caps_with_plus_off_the_mac, unwired_keys_show_their_source, highlight_marks_every_match);
