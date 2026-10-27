// ListBox, after ely-core/src/forms/listbox.rs.
#[macro_use]
mod common;
use common::*;
use slint::{ComponentHandle, Model};
use slint::platform::Key;

slint::slint! {
    import { ListBox, Choice, Choices, Toggles } from "ui/index.slint";
    export { Choice, Choices, Toggles }
    export component Harness inherits Window {
        width: 400px;
        height: 400px;
        in-out property <[string]> role: ["member"];
        in-out property <bool> multiple;
        out property <int> highlighted: list.highlighted;
        out property <bool> focused: list.has-focus;
        public function focus-list() { list.focus(); }
        list := ListBox {
            x: 10px;
            y: 10px;
            width: 200px;
            multiple: root.multiple;
            choices: [
                { value: "admin", label: "Admin" },
                { value: "member", label: "Member" },
                { value: "guest", label: "Guest", disabled: true },
                { value: "owner", label: "Owner" },
            ];
            selected <=> root.role;
        }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    ely_slint::wire_options!(h);
    ely_slint::wire_toggles!(h);
    show(&h);
    h
}

fn role(h: &Harness) -> Vec<String> {
    h.get_role().iter().map(|l| l.to_string()).collect()
}

fn a_click_chooses_one_and_focuses() {
    let h = harness();
    // First row: 1 px border, 4 px in, about 30 px tall.
    click(h.window(), 60.0, 28.0);
    assert_eq!(role(&h), ["admin"]);
    assert!(h.get_focused());
}

fn keys_walk_and_choose() {
    let h = harness();
    h.invoke_focus_list();
    tap(h.window(), Key::DownArrow);
    tap(h.window(), Key::DownArrow);
    assert_eq!(h.get_highlighted(), 3, "Guest is disabled");
    tap(h.window(), Key::Return);
    assert_eq!(role(&h), ["owner"]);
    tap(h.window(), Key::Home);
    assert_eq!(h.get_highlighted(), 0);
}

fn multiple_toggles_in_order() {
    let h = harness();
    h.set_multiple(true);
    h.invoke_focus_list();
    tap(h.window(), " ");
    assert_eq!(role(&h), ["admin", "member"]);
    tap(h.window(), Key::DownArrow);
    tap(h.window(), " ");
    assert_eq!(role(&h), ["admin"]);
}

cases!(list_box: a_click_chooses_one_and_focuses, keys_walk_and_choose, multiple_toggles_in_order);
