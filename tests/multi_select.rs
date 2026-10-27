// MultiSelect, after ely-core/src/forms/multi.rs.
#[macro_use]
mod common;
use common::*;
use slint::{ComponentHandle, Model};
use slint::platform::Key;

slint::slint! {
    import { MultiSelect, Choice, Choices, Toggles, OptionFloat, OptionLayer } from "ui/index.slint";
    export { Choice, Choices, Toggles }
    export component Harness inherits Window {
        width: 400px;
        height: 400px;
        in-out property <[string]> labels: ["docs"];
        in-out property <int> changes;
        out property <bool> open: multi.open;
        out property <int> highlighted: multi.highlighted;
        out property <bool> focused: multi.has-focus;
        out property <length> multi-height: multi.height;
        public function focus-multi() { multi.focus(); }
        multi := MultiSelect {
            x: 10px;
            y: 10px;
            width: 300px;
            choices: [
                { value: "bug", label: "Bug" },
                { value: "docs", label: "Docs" },
                { value: "wip", label: "WIP", disabled: true },
                { value: "ui", label: "UI" },
            ];
            selected <=> root.labels;
            selection-changed => { root.changes += 1; }
        }
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

fn labels(h: &Harness) -> Vec<String> {
    h.get_labels().iter().map(|l| l.to_string()).collect()
}

fn a_click_opens_at_the_first_choice() {
    let h = harness();
    click(h.window(), 250.0, 24.0);
    advance(0);
    assert!(h.get_focused());
    assert!(h.get_open());
    assert_eq!(h.get_highlighted(), 1, "Docs, the first chosen");
}

fn enter_toggles_and_keeps_it_open() {
    let h = harness();
    h.invoke_focus_multi();
    tap(h.window(), Key::DownArrow);
    tap(h.window(), Key::DownArrow);
    assert_eq!(h.get_highlighted(), 3, "WIP is disabled");
    tap(h.window(), Key::Return);
    advance(0);
    assert!(h.get_open());
    assert_eq!(labels(&h), ["docs", "ui"], "choice order");
    tap(h.window(), " ");
    assert_eq!(labels(&h), ["docs"]);
    tap(h.window(), Key::Escape);
    assert!(!h.get_open());
}

fn backspace_drops_the_last_chosen() {
    let h = harness();
    h.set_labels(slint::ModelRc::new(slint::VecModel::from(vec!["ui".into(), "bug".into()])));
    h.invoke_focus_multi();
    tap(h.window(), Key::Backspace);
    assert_eq!(labels(&h), ["ui"]);
    assert_eq!(h.get_changes(), 1);
}

fn chips_wrap_and_grow_the_field() {
    let h = harness();
    let one_line = h.get_multi_height();
    // A 24 px chip, 2 px above and below, a 1 px border: past the 28 px least, as in gpui.
    assert_eq!(one_line, 30.0);
    h.set_labels(slint::ModelRc::new(slint::VecModel::from(vec!["bug".into(), "docs".into(), "ui".into(), "wip".into()])));
    advance(0);
    assert!(h.get_multi_height() >= one_line);
}

cases!(multi_select: a_click_opens_at_the_first_choice, enter_toggles_and_keeps_it_open, backspace_drops_the_last_chosen, chips_wrap_and_grow_the_field);
