// Select, after ely-core/src/forms/select.rs.
#[macro_use]
mod common;
use common::*;
use slint::ComponentHandle;
use slint::platform::Key;

slint::slint! {
    import { Select, Choice, Choices, Toggles, OptionFloat, OptionLayer } from "ui/index.slint";
    export { Choice, Choices, Toggles }
    export component Harness inherits Window {
        width: 400px;
        height: 400px;
        in-out property <string> plan;
        in-out property <int> changes;
        out property <bool> open: sel.open;
        out property <int> highlighted: sel.highlighted;
        out property <bool> focused: sel.has-focus;
        out property <int> rows: OptionFloat.owner == "" ? 0 : OptionFloat.rows.length;
        out property <int> checked: OptionFloat.checked.length;
        public function focus-sel() { sel.focus(); }
        sel := Select {
            x: 10px;
            y: 10px;
            width: 200px;
            choices: [
                { value: "free", label: "Free" },
                { value: "team", label: "Team", disabled: true },
                { value: "pro", label: "Pro" },
            ];
            selected <=> root.plan;
            selection-changed => { root.changes += 1; }
        }
        other := FocusScope { x: 300px; y: 300px; width: 20px; height: 20px; }
        public function focus-other() { other.focus(); }
        OptionLayer { }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    ely_slint::wire_options!(h);
    ely_slint::wire_toggles!(h);
    show(&h);
    h
}

fn a_press_opens_and_focuses() {
    let h = harness();
    click(h.window(), 50.0, 24.0);
    advance(0);
    assert!(h.get_open());
    assert!(h.get_focused());
    assert_eq!(h.get_rows(), 3);
    assert_eq!(h.get_highlighted(), 0, "the first enabled row");
}

fn keys_walk_past_disabled_and_pick() {
    let h = harness();
    h.invoke_focus_sel();
    tap(h.window(), Key::DownArrow);
    assert!(h.get_open());
    tap(h.window(), Key::DownArrow);
    assert_eq!(h.get_highlighted(), 2, "Team is disabled");
    tap(h.window(), Key::Return);
    advance(0);
    assert!(!h.get_open());
    assert_eq!(h.get_plan(), "pro");
    assert_eq!(h.get_changes(), 1);
    tap(h.window(), " ");
    advance(0);
    assert_eq!(h.get_highlighted(), 2, "opens at the choice");
    assert_eq!(h.get_checked(), 1);
    tap(h.window(), Key::Home);
    assert_eq!(h.get_highlighted(), 0);
    tap(h.window(), Key::End);
    assert_eq!(h.get_highlighted(), 2);
    tap(h.window(), Key::Escape);
    assert!(!h.get_open());
}

fn a_row_press_picks() {
    let h = harness();
    click(h.window(), 50.0, 24.0);
    advance(0);
    // The list opens under the 28 px field: rows from about y 46.
    click(h.window(), 50.0, 52.0);
    advance(0);
    assert_eq!(h.get_plan(), "free");
    assert!(!h.get_open());
}

fn leaving_or_pressing_outside_closes() {
    let h = harness();
    click(h.window(), 50.0, 24.0);
    advance(0);
    h.invoke_focus_other();
    advance(0);
    assert!(!h.get_open(), "blur closes");
    h.invoke_focus_sel();
    tap(h.window(), Key::DownArrow);
    click(h.window(), 350.0, 200.0);
    advance(0);
    assert!(!h.get_open(), "a press outside closes");
}

cases!(select: a_press_opens_and_focuses, keys_walk_past_disabled_and_pick, a_row_press_picks, leaving_or_pressing_outside_closes);
