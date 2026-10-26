#[macro_use]
mod common;
use common::*;
use slint::platform::Key;

slint::slint! {
    import { KeyStrokes, Theme, PlatformStyle } from "ui/index.slint";
    export component Harness inherits Window {
        width: 200px;
        height: 100px;
        in-out property <string> last;
        in-out property <PlatformStyle> platform <=> Theme.platform;
        forward-focus: keys;
        keys := FocusScope {
            key-pressed(event) => {
                if !KeyStrokes.is-modifier(event.text) {
                    root.last = KeyStrokes.unparse(event);
                }
                accept
            }
        }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    show(&h);
    h
}

fn press_with(h: &Harness, mods: &[Key], key: &str) {
    for m in mods {
        key_down(h.window(), *m);
    }
    key_down(h.window(), key);
    key_up(h.window(), key);
    for m in mods.iter().rev() {
        key_up(h.window(), *m);
    }
}

fn letters_lower_case_with_shift_named() {
    let h = harness();
    press_with(&h, &[], "a");
    assert_eq!(h.get_last(), "a");
    press_with(&h, &[Key::Shift], "A");
    assert_eq!(h.get_last(), "shift-a");
}

// gpui order: fn, ctrl, alt, platform, shift.
fn modifiers_in_gpui_order() {
    let h = harness();
    h.set_platform(PlatformStyle::Linux);
    press_with(&h, &[Key::Shift, Key::Alt, Key::Control], "K");
    assert_eq!(h.get_last(), "ctrl-alt-shift-k");
    press_with(&h, &[Key::Meta], "s");
    assert_eq!(h.get_last(), "super-s");
    h.set_platform(PlatformStyle::Windows);
    press_with(&h, &[Key::Meta], "s");
    assert_eq!(h.get_last(), "win-s");
}

// Slint reports Command as control on macOS; gpui calls it cmd.
fn mac_swaps_command_and_control() {
    let h = harness();
    h.set_platform(PlatformStyle::Mac);
    press_with(&h, &[Key::Control], "s");
    assert_eq!(h.get_last(), "cmd-s");
    press_with(&h, &[Key::Meta], "s");
    assert_eq!(h.get_last(), "ctrl-s");
}

fn named_keys() {
    let h = harness();
    h.set_platform(PlatformStyle::Linux);
    for (key, name) in [(Key::Return, "enter"), (Key::Escape, "escape"), (Key::LeftArrow, "left"), (Key::F5, "f5"), (Key::Space, "space"), (Key::PageDown, "pagedown")] {
        press_with(&h, &[], &slint::SharedString::from(key));
        assert_eq!(h.get_last(), name);
    }
}

fn modifier_alone_reports_nothing() {
    let h = harness();
    h.set_last("—".into());
    press_with(&h, &[], &slint::SharedString::from(Key::Shift));
    assert_eq!(h.get_last(), "—");
}

cases!(keystroke: letters_lower_case_with_shift_named, modifiers_in_gpui_order, mac_swaps_command_and_control, named_keys, modifier_alone_reports_nothing);
