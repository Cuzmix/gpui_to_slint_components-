#[macro_use]
mod common;
use common::*;

slint::slint! {
    import { Icon, Icons, IconSize, Spinner, SpinnerStyle, Theme } from "ui/index.slint";
    export component Harness inherits Window {
        width: 200px;
        height: 100px;
        in-out property <IconSize> isize: md;
        in-out property <SpinnerStyle> kind: ring;
        out property <length> icon-w: icon.width;
        out property <float> phase: spin.phase;
        out property <length> spin-w: spin.width;
        public function reduce(on: bool) { Theme.reduced-motion = on; }
        public function font-scale(f: float) { Theme.font-scale = f; }
        HorizontalLayout {
            icon := Icon { source: Icons.check; size: isize; }
            spin := Spinner { kind: kind; }
        }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    show(&h);
    h.invoke_reduce(false);
    h.invoke_font_scale(1.0);
    h
}

fn icon_sizes_follow_the_scale() {
    let h = harness();
    for (size, px) in [(IconSize::Xs, 12.0), (IconSize::Sm, 14.0), (IconSize::Md, 16.0), (IconSize::Lg, 20.0), (IconSize::Xl, 24.0), (IconSize::Xxl, 32.0)] {
        h.set_isize(size);
        assert_eq!(h.get_icon_w(), px);
    }
    h.invoke_font_scale(1.5);
    h.set_isize(IconSize::Md);
    assert_eq!(h.get_icon_w(), 24.0);
}

fn spinner_periods_match_source() {
    let h = harness();
    for (kind, period) in [(SpinnerStyle::Ring, 900u64), (SpinnerStyle::Dots, 1200), (SpinnerStyle::Bars, 1000), (SpinnerStyle::Pulse, 1400), (SpinnerStyle::Orbit, 1100), (SpinnerStyle::Wave, 1200)] {
        h.set_kind(kind);
        let a = h.get_phase();
        advance(period);
        let b = h.get_phase();
        assert!((a - b).abs() < 1e-3 || (1.0 - (a - b).abs()) < 1e-3, "{kind:?}: {a} -> {b}");
        advance(period / 4);
        let c = h.get_phase();
        let moved = (c - b).rem_euclid(1.0);
        assert!((moved - 0.25).abs() < 0.01, "{kind:?}: quarter period moved {moved}");
    }
    assert_eq!(h.get_spin_w(), 16.0);
}

fn reduced_motion_holds_the_spinner_at_rest() {
    let h = harness();
    h.invoke_reduce(true);
    assert!((h.get_phase() - 0.3).abs() < 1e-6);
    advance(450);
    assert!((h.get_phase() - 0.3).abs() < 1e-6);
}

cases!(primitives: icon_sizes_follow_the_scale, spinner_periods_match_source, reduced_motion_holds_the_spinner_at_rest);
