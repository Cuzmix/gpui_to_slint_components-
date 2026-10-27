// HotkeyInput, after ely-core/src/forms/hotkey.rs.
#[macro_use]
mod common;
use common::*;
use slint::ComponentHandle;
use slint::platform::Key;

slint::slint! {
    import { HotkeyInput, Theme, PlatformStyle, TextFormat } from "ui/index.slint";
    export { TextFormat }
    export component Harness inherits Window {
        width: 400px;
        height: 200px;
        in-out property <string> value: "ctrl-s";
        in-out property <int> changes;
        out property <bool> recording: hot.recording;
        public function focus-hot() { hot.focus(); }
        init => { Theme.platform = PlatformStyle.linux; }
        hot := HotkeyInput {
            x: 10px;
            y: 10px;
            width: 300px;
            value: root.value;
            label: "Save";
            value-changed(v) => { root.value = v; root.changes += 1; }
        }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    show(&h);
    h
}

fn chord(h: &Harness, held: &[Key], key: &str) {
    for k in held {
        key_down(h.window(), *k);
    }
    tap(h.window(), key);
    for k in held.iter().rev() {
        key_up(h.window(), *k);
    }
}

fn a_press_starts_recording() {
    let h = harness();
    click(h.window(), 100.0, 26.0);
    assert!(h.get_recording());
}

fn records_one_shortcut_then_lets_go() {
    let h = harness();
    h.invoke_focus_hot();
    chord(&h, &[Key::Control, Key::Shift], "K");
    assert_eq!(h.get_value(), "ctrl-shift-k");
    assert_eq!(h.get_changes(), 1, "modifiers alone record nothing");
    assert!(!h.get_recording(), "focus leaves");
}

fn backspace_clears_and_escape_leaves() {
    let h = harness();
    h.invoke_focus_hot();
    tap(h.window(), Key::Escape);
    assert!(!h.get_recording());
    assert_eq!(h.get_value(), "ctrl-s");
    assert_eq!(h.get_changes(), 0);
    h.invoke_focus_hot();
    tap(h.window(), Key::Backspace);
    assert_eq!(h.get_value(), "");
    assert_eq!(h.get_changes(), 1);
}

fn modified_escape_and_tab_are_recorded() {
    let h = harness();
    h.invoke_focus_hot();
    chord(&h, &[Key::Shift], slint::SharedString::from(Key::Escape).as_str());
    assert_eq!(h.get_value(), "shift-escape");
    h.invoke_focus_hot();
    tap(h.window(), Key::Tab);
    assert_eq!(h.get_value(), "tab");
}

cases!(hotkey_input: a_press_starts_recording, records_one_shortcut_then_lets_go, backspace_clears_and_escape_leaves, modified_escape_and_tab_are_recorded);
