#[macro_use]
mod common;

slint::slint! {
    import { Theme, Title, Subtitle, Paragraph, Label, Caption, Overline, Heading, HeadingLevel } from "ui/index.slint";
    export component Harness inherits Window {
        width: 600px;
        height: 400px;
        in-out property <HeadingLevel> level: HeadingLevel.h1;
        out property <length> title-size: title.font-size;
        out property <int> title-weight: title.font-weight;
        out property <bool> title-ink: title.color == Theme.colors.fg;
        out property <length> title-line: title.preferred-height;
        out property <length> sub-size: sub.font-size;
        out property <int> sub-weight: sub.font-weight;
        out property <bool> sub-ink: sub.color == Theme.colors.fg-muted;
        out property <length> sub-line: sub.preferred-height;
        out property <length> para-size: para.font-size;
        out property <int> para-weight: para.font-weight;
        out property <bool> para-ink: para.color == Theme.colors.fg;
        out property <length> para-line: para.preferred-height;
        out property <length> para-wrapped: wrapped.preferred-height;
        out property <length> label-size: label.font-size;
        out property <int> label-weight: label.font-weight;
        out property <bool> label-ink: label.color == Theme.colors.fg;
        out property <length> label-line: label.preferred-height;
        out property <length> cap-size: cap.font-size;
        out property <int> cap-weight: cap.font-weight;
        out property <bool> cap-ink: cap.color == Theme.colors.fg-muted;
        out property <length> cap-line: cap.preferred-height;
        out property <length> over-line: over.preferred-height;
        out property <length> head-size: head.font-size;
        out property <int> head-weight: head.font-weight;
        out property <bool> head-ink: head.color == Theme.colors.fg;
        out property <length> head-line: head.preferred-height;
        title := Title { text: "Settings"; }
        sub := Subtitle { y: 40px; text: "Manage your account"; }
        para := Paragraph { y: 80px; text: "Reading text"; }
        wrapped := Paragraph { y: 120px; width: 60px; text: "Reading text that wraps"; }
        label := Label { y: 200px; text: "Email"; }
        cap := Caption { y: 230px; text: "Updated just now"; }
        over := Overline { y: 260px; text: "Billing"; }
        head := Heading { y: 300px; level: root.level; text: "Payment method"; }
    }
}

fn near(a: f32, b: f32) {
    assert!((a - b).abs() < 0.05, "{a} is not {b}");
}

// One line's height as GPUI lays it out at scale 1: rounded to a whole pixel, half down.
fn line(size: f32, leading: f32) -> f32 {
    (size * leading - 0.5).ceil()
}

fn text_kinds_take_the_source_specs() {
    let h = Harness::new().unwrap();
    common::show(&h);
    assert_eq!((h.get_title_size(), h.get_title_weight(), h.get_title_ink()), (20.0, 600, true));
    assert_eq!((h.get_sub_size(), h.get_sub_weight(), h.get_sub_ink()), (16.0, 400, true));
    assert_eq!((h.get_para_size(), h.get_para_weight(), h.get_para_ink()), (14.0, 400, true));
    assert_eq!((h.get_label_size(), h.get_label_weight(), h.get_label_ink()), (13.0, 500, true));
    assert_eq!((h.get_cap_size(), h.get_cap_weight(), h.get_cap_ink()), (12.0, 400, true));
    assert_eq!((h.get_head_size(), h.get_head_weight(), h.get_head_ink()), (32.0, 600, true));
}

fn leading_is_a_multiple_of_the_font_size() {
    let h = Harness::new().unwrap();
    common::show(&h);
    near(h.get_title_line(), line(20.0, 1.3));
    near(h.get_sub_line(), line(16.0, 1.45));
    near(h.get_para_line(), line(14.0, 1.6));
    near(h.get_label_line(), line(13.0, 1.4));
    near(h.get_cap_line(), line(12.0, 1.45));
    near(h.get_over_line(), line(11.0, 1.4));
    near(h.get_head_line(), line(32.0, 1.25));
}

fn paragraphs_wrap_line_by_line() {
    let h = Harness::new().unwrap();
    common::show(&h);
    let lines = h.get_para_wrapped() / line(14.0, 1.6);
    assert!(lines >= 2.0 && (lines - lines.round()).abs() < 0.01, "{lines} lines");
}

fn heading_levels_run_display_to_base() {
    let h = Harness::new().unwrap();
    common::show(&h);
    let sizes = [32.0, 24.0, 20.0, 16.0, 14.0, 13.0];
    let levels = [HeadingLevel::H1, HeadingLevel::H2, HeadingLevel::H3, HeadingLevel::H4, HeadingLevel::H5, HeadingLevel::H6];
    for (level, size) in levels.into_iter().zip(sizes) {
        h.set_level(level);
        assert_eq!(h.get_head_size(), size);
        near(h.get_head_line(), line(size, 1.25));
    }
}

fn overline_sets_upper_case() {
    let h = Harness::new().unwrap();
    common::show(&h);
    let texts: Vec<_> = i_slint_backend_testing::ElementHandle::find_by_accessible_label(&h, "BILLING").collect();
    assert_eq!(texts.len(), 1);
}

cases!(typography: text_kinds_take_the_source_specs, leading_is_a_multiple_of_the_font_size, paragraphs_wrap_line_by_line, heading_levels_run_display_to_base, overline_sets_upper_case);
