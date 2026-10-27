// TransferList, after ely-core/src/forms/transfer.rs.
#[macro_use]
mod common;
use common::*;
use slint::{ComponentHandle, Model};

slint::slint! {
    import { TransferList, Choice, Choices, Toggles } from "ui/index.slint";
    export { Choice, Choices, Toggles }
    export component Harness inherits Window {
        width: 600px;
        height: 400px;
        in-out property <[string]> columns: ["email"];
        in-out property <int> moves;
        TransferList {
            x: 10px;
            y: 10px;
            width: 500px;
            choices: [
                { value: "name", label: "Name" },
                { value: "email", label: "Email" },
                { value: "phone", label: "Phone" },
            ];
            chosen <=> root.columns;
            chosen-changed => { root.moves += 1; }
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

fn columns(h: &Harness) -> Vec<String> {
    h.get_columns().iter().map(|l| l.to_string()).collect()
}

fn mark_then_move_keeps_choice_order() {
    let h = harness();
    // Left holds Name and Phone; mark Phone, then Name.
    click_label(&h, "Phone");
    click_label(&h, "Name");
    click_label(&h, "Move to chosen");
    assert_eq!(columns(&h), ["name", "email", "phone"]);
    assert_eq!(h.get_moves(), 1);
}

fn moving_back_and_nothing_marked() {
    let h = harness();
    click_label(&h, "Move back");
    assert_eq!(h.get_moves(), 0, "nothing marked: the button is disabled");
    click_label(&h, "Email");
    click_label(&h, "Move back");
    assert!(columns(&h).is_empty());
}

cases!(transfer_list: mark_then_move_keeps_choice_order, moving_back_and_nothing_marked);
