#[macro_use]
mod common;
use common::*;
use slint::Color;

slint::slint! {
    import { Theme, ThemeFader, Mode, ControlSize, Density, Radius, IconSize, TextSize, Elevation } from "ui/index.slint";
    export component Harness inherits Window {
        width: 100px;
        height: 100px;
        out property <color> bg: Theme.colors.bg;
        out property <color> fg: Theme.colors.fg;
        out property <color> border: Theme.colors.border;
        out property <color> chart0: Theme.colors.chart[0];
        out property <color> ansi15: Theme.colors.ansi[15];
        out property <color> keyword: Theme.colors.syntax.keyword;
        out property <float> fade: Theme.fade;
        out property <length> md-height: Theme.control-height(ControlSize.md);
        out property <length> md-padding: Theme.control-padding(ControlSize.md);
        out property <length> radius-lg: Theme.radius(Radius.lg);
        out property <length> icon-xl: Theme.icon-size(IconSize.xl);
        out property <length> text-base: Theme.text-size(TextSize.base);
        out property <length> modal-far-blur: Theme.shadow-far(Elevation.modal).blur;
        out property <color> modal-far-color: Theme.shadow-far(Elevation.modal).color;
        public pure function spring(t: float) -> float { Theme.spring(t) }
        public function set-mode(m: Mode) { Theme.mode = m; }
        public function set-mode-now(m: Mode) { Theme.set-mode-now(m); }
        public function set-density(d: Density) { Theme.density = d; }
        public function set-hc(on: bool) { Theme.high-contrast = on; }
        public function set-cb(on: bool) { Theme.color-blind-safe = on; }
        public function set-font-scale(f: float) { Theme.font-scale = f; }
        public function set-radius-scale(f: float) { Theme.radius-scale = f; }
        public function set-reduced(on: bool) { Theme.reduced-motion = on; }
        ThemeFader {}
    }
    export component NoFader inherits Window {
        out property <color> bg: Theme.colors.bg;
        public function set-mode(m: Mode) { Theme.mode = m; }
    }
}

fn settle() {
    advance(1);
    advance(400);
}

fn hex(c: u32) -> Color {
    Color::from_rgb_u8((c >> 16) as u8, (c >> 8) as u8, c as u8)
}

fn reset(h: &Harness) {
    h.invoke_set_mode_now(Mode::Light);
    h.invoke_set_hc(false);
    h.invoke_set_cb(false);
    h.invoke_set_density(Density::Standard);
    h.invoke_set_font_scale(1.0);
    h.invoke_set_radius_scale(1.0);
    h.invoke_set_reduced(false);
    settle();
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    show(&h);
    reset(&h);
    h
}

fn light_palette_values_match_source() {
    let h = harness();
    assert_eq!(h.get_bg(), hex(0xfcfaf7));
    assert_eq!(h.get_fg(), hex(0x181613));
    assert_eq!(h.get_chart0(), hex(0x5182c1));
    assert_eq!(h.get_ansi15(), hex(0xcccac8));
    assert_eq!(h.get_keyword(), hex(0x714ca6));
}

fn mode_change_fades_over_280ms_with_ease_in_out_cubic() {
    let h = harness();
    h.invoke_set_mode(Mode::Dark);
    advance(1);
    let start = h.get_bg();
    assert_eq!(start, hex(0xfcfaf7), "fade starts from the light ground");
    advance(140);
    let mid = h.get_bg();
    // ease_in_out_cubic(0.5) = 0.5: halfway between fcfaf7 and 252525, within a frame.
    let r = mid.red() as i32;
    assert!((r - 0x90).abs() <= 8, "mid red {r:#x}");
    advance(200);
    assert_eq!(h.get_bg(), hex(0x252525));
    assert_eq!(h.get_fade(), 1.0);
}

fn set_mode_now_skips_the_fade() {
    let h = harness();
    h.invoke_set_mode_now(Mode::Dark);
    assert_eq!(h.get_bg(), hex(0x252525));
}

fn without_fader_changes_apply_at_once() {
    let h = NoFader::new().unwrap();
    show(&h);
    h.invoke_set_mode(Mode::Dark);
    assert_eq!(h.get_bg(), hex(0x252525));
}

fn high_contrast_strengthens_borders() {
    let h = harness();
    h.invoke_set_hc(true);
    settle();
    assert_eq!(h.get_border(), hex(0xa6a4a2));
    h.invoke_set_mode_now(Mode::Dark);
    assert_eq!(h.get_border(), hex(0x6a6966));
}

fn color_blind_safe_swaps_chart_hues_only() {
    let h = harness();
    h.invoke_set_cb(true);
    settle();
    assert_eq!(h.get_chart0(), hex(0x0240b1));
    assert_eq!(h.get_bg(), hex(0xfcfaf7));
}

fn measures_follow_density_and_scales() {
    let h = harness();
    assert_eq!(h.get_md_height(), 28.0);
    assert_eq!(h.get_md_padding(), 12.0);
    h.invoke_set_density(Density::Compact);
    assert_eq!(h.get_md_height(), 24.0);
    assert_eq!(h.get_md_padding(), 10.0);
    h.invoke_set_density(Density::Comfortable);
    assert_eq!(h.get_md_height(), 32.0);
    assert_eq!(h.get_md_padding(), 14.0);
    assert_eq!(h.get_radius_lg(), 8.0);
    h.invoke_set_radius_scale(1.5);
    assert_eq!(h.get_radius_lg(), 12.0);
    assert_eq!(h.get_icon_xl(), 24.0);
    assert_eq!(h.get_text_base(), 13.0);
    h.invoke_set_font_scale(2.0);
    assert_eq!(h.get_icon_xl(), 48.0);
    assert_eq!(h.get_text_base(), 26.0);
}

fn modal_shadow_layers_match_source() {
    let h = harness();
    assert_eq!(h.get_modal_far_blur(), 40.0);
    let c = h.get_modal_far_color();
    assert_eq!((c.red(), c.green(), c.blue()), (0x18, 0x16, 0x13));
    assert!((c.alpha() as i32 - (0.14f32 * 255.0).round() as i32).abs() <= 1);
    h.invoke_set_mode_now(Mode::Dark);
    assert_eq!(h.get_modal_far_blur(), 48.0);
}

fn reduced_motion_cuts_the_fade_short() {
    let h = harness();
    h.invoke_set_reduced(true);
    h.invoke_set_mode(Mode::Dark);
    advance(1);
    advance(16);
    assert_eq!(h.get_bg(), hex(0x252525));
}

// The spring overshoots a little, then settles, as gpui's `motion::spring` (its own test).
fn the_spring_overshoots_a_little_then_settles() {
    init();
    let h = Harness::new().unwrap();
    assert_eq!(h.invoke_spring(0.0), 0.0);
    assert_eq!(h.invoke_spring(1.0), 1.0);
    let peak = (0..=100).map(|i| h.invoke_spring(i as f32 / 100.0)).fold(0.0, f32::max);
    assert!(peak > 1.01 && peak < 1.05, "peak {peak}");
    assert!((h.invoke_spring(0.95) - 1.0).abs() < 0.01);
}

cases!(theme: the_spring_overshoots_a_little_then_settles, light_palette_values_match_source, mode_change_fades_over_280ms_with_ease_in_out_cubic, set_mode_now_skips_the_fade, without_fader_changes_apply_at_once, high_contrast_strengthens_borders, color_blind_safe_swaps_chart_hues_only, measures_follow_density_and_scales, modal_shadow_layers_match_source, reduced_motion_cuts_the_fade_short);
