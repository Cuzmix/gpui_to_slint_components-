// CascadeSelect, after ely-core/src/forms/cascade.rs.
#[macro_use]
mod common;
use common::*;
use slint::platform::Key;
use slint::{ComponentHandle, Model};

slint::slint! {
    import { CascadeSelect, CascadeNode, Cascades, Choice, Choices, OptionFloat, OptionLayer } from "ui/index.slint";
    export { CascadeNode, Cascades, Choice, Choices }
    export component Harness inherits Window {
        width: 600px;
        height: 400px;
        in-out property <[string]> place;
        in-out property <int> changes;
        out property <bool> open: pick.open;
        out property <[int]> trail: pick.trail;
        out property <bool> focused: pick.has-focus;
        out property <int> columns: OptionFloat.owner == "" ? 0 : OptionFloat.columns.length;
        out property <int> deepest: OptionFloat.deepest;
        out property <string> value: pick.accessible-value;
        public function focus-pick() { pick.focus(); }
        pick := CascadeSelect {
            x: 10px;
            y: 10px;
            width: 200px;
            nodes: [
                { value: "asia", label: "Asia", parent: -1 },
                { value: "jp", label: "Japan", parent: 0 },
                { value: "tyo", label: "Tokyo", parent: 1 },
                { value: "osa", label: "Osaka", parent: 1 },
                { value: "eu", label: "Europe", parent: -1 },
                { value: "pt", label: "Portugal", parent: 4 },
            ];
            selected <=> root.place;
            selection-changed => { root.changes += 1; }
        }
        other := FocusScope { x: 500px; y: 300px; width: 20px; height: 20px; }
        public function focus-other() { other.focus(); }
        OptionLayer { }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    ely_slint::wire_options!(h);
    ely_slint::wire_cascades!(h);
    show(&h);
    h
}

fn trail(h: &Harness) -> Vec<i32> {
    h.get_trail().iter().collect()
}

fn place(h: &Harness) -> Vec<String> {
    h.get_place().iter().map(|s| s.to_string()).collect()
}

// Moves the pointer over the middle of the first element named `label`.
fn hover_label(h: &Harness, label: &str) {
    let e = i_slint_backend_testing::ElementHandle::find_by_accessible_label(h, label).next().unwrap_or_else(|| panic!("an element named {label}"));
    let (at, size) = (e.absolute_position(), e.size());
    mouse_move(h.window(), at.x + size.width / 2.0, at.y + size.height / 2.0);
    advance(0);
}

fn a_press_opens_the_roots() {
    let h = harness();
    click(h.window(), 50.0, 24.0);
    advance(0);
    assert!(h.get_open());
    assert!(h.get_focused());
    assert_eq!(trail(&h), [0]);
    assert_eq!(h.get_columns(), 2, "the roots, and Asia's countries");
    click(h.window(), 50.0, 24.0);
    advance(0);
    assert!(!h.get_open(), "a second press closes");
}

fn keys_walk_columns_and_pick_a_leaf() {
    let h = harness();
    h.invoke_focus_pick();
    tap(h.window(), Key::DownArrow);
    assert!(h.get_open());
    assert_eq!(trail(&h), [0]);
    tap(h.window(), Key::UpArrow);
    assert_eq!(trail(&h), [1], "wraps to Europe");
    tap(h.window(), Key::DownArrow);
    assert_eq!(trail(&h), [0]);
    tap(h.window(), Key::RightArrow);
    assert_eq!(trail(&h), [0, 0]);
    assert_eq!(h.get_columns(), 3);
    assert_eq!(h.get_deepest(), 2);
    tap(h.window(), Key::Return);
    assert_eq!(trail(&h), [0, 0, 0], "Enter on a branch steps in");
    tap(h.window(), Key::DownArrow);
    assert_eq!(trail(&h), [0, 0, 1]);
    tap(h.window(), Key::LeftArrow);
    assert_eq!(trail(&h), [0, 0]);
    tap(h.window(), Key::RightArrow);
    tap(h.window(), Key::DownArrow);
    tap(h.window(), " ");
    advance(0);
    assert!(!h.get_open());
    assert_eq!(place(&h), ["asia", "jp", "osa"]);
    assert_eq!(h.get_changes(), 1);
    assert_eq!(h.get_value(), "Asia / Japan / Osaka");
    tap(h.window(), Key::Return);
    assert_eq!(trail(&h), [0, 0, 1], "opens at the choice");
    tap(h.window(), Key::Escape);
    assert!(!h.get_open());
}

fn hovering_opens_and_pressing_a_leaf_picks() {
    let h = harness();
    click(h.window(), 50.0, 24.0);
    advance(0);
    hover_label(&h, "Europe");
    assert_eq!(trail(&h), [1]);
    hover_label(&h, "Portugal");
    assert_eq!(trail(&h), [1, 0]);
    hover_label(&h, "Asia");
    click_label(&h, "Asia");
    advance(0);
    assert!(h.get_open(), "a branch does not pick");
    assert_eq!(trail(&h), [0]);
    hover_label(&h, "Japan");
    hover_label(&h, "Tokyo");
    click_label(&h, "Tokyo");
    advance(0);
    assert!(!h.get_open());
    assert_eq!(place(&h), ["asia", "jp", "tyo"]);
}

fn leaving_or_pressing_outside_closes() {
    let h = harness();
    click(h.window(), 50.0, 24.0);
    advance(0);
    h.invoke_focus_other();
    advance(0);
    assert!(!h.get_open(), "blur closes");
    h.invoke_focus_pick();
    tap(h.window(), Key::DownArrow);
    click(h.window(), 550.0, 380.0);
    advance(0);
    assert!(!h.get_open(), "a press outside closes");
}

cases!(cascade: a_press_opens_the_roots, keys_walk_columns_and_pick_a_leaf, hovering_opens_and_pressing_a_leaf_picks, leaving_or_pressing_outside_closes);
