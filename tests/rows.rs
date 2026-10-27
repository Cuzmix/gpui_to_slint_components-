// FieldArray, KeyValueInput and ListInput, after ely-core/src/forms/pairs.rs.
#[macro_use]
mod common;
use common::*;
use i_slint_backend_testing::{AccessibleRole, ElementHandle};
use slint::{ComponentHandle, Model};

slint::slint! {
    import { KeyValueInput, ListInput, KeyValue, FieldRows, TextFields, TextFit } from "ui/index.slint";
    export { FieldRows, KeyValue, TextFields, TextFit }
    export component Harness inherits Window {
        width: 500px;
        height: 400px;
        in-out property <[KeyValue]> pairs: [{ key: "Accept", value: "text/html" }];
        in-out property <[string]> items: ["rust"];
        in-out property <int> pair-reports;
        in-out property <int> item-reports;
        VerticalLayout {
            alignment: start;
            spacing: 16px;
            KeyValueInput {
                pairs <=> root.pairs;
                pairs-changed => { root.pair-reports += 1; }
            }
            ListInput {
                items <=> root.items;
                placeholder: "Tag";
                items-changed => { root.item-reports += 1; }
            }
        }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    ely_slint::wire_fields!(h);
    ely_slint::wire_rows!(h);
    show(&h);
    h
}

fn button(h: &Harness, label: &str) -> ElementHandle {
    ElementHandle::find_by_accessible_label(h, label).find(|e| e.accessible_role() == Some(AccessibleRole::Button)).unwrap()
}

fn pairs(h: &Harness) -> Vec<(String, String)> {
    h.get_pairs().iter().map(|pair| (pair.key.to_string(), pair.value.to_string())).collect()
}

fn adding_a_pair_focuses_its_key() {
    let h = harness();
    button(&h, "Add pair").invoke_accessible_default_action();
    assert_eq!(pairs(&h).len(), 2);
    assert_eq!(h.get_pair_reports(), 1);
    tap(h.window(), "X");
    assert_eq!(pairs(&h)[1], ("X".to_string(), String::new()));
    assert_eq!(h.get_pair_reports(), 2);
}

fn editing_a_value_reports_every_pair() {
    let h = harness();
    let value = ElementHandle::find_by_accessible_label(&h, "Value").find(|e| e.accessible_role() == Some(AccessibleRole::TextInput)).unwrap();
    let (at, size) = (value.absolute_position(), value.size());
    click(h.window(), at.x + size.width - 2.0, at.y + size.height / 2.0);
    tap(h.window(), "!");
    assert_eq!(pairs(&h), [("Accept".to_string(), "text/html!".to_string())]);
}

fn removing_rows() {
    let h = harness();
    button(&h, "Add item").invoke_accessible_default_action();
    tap(h.window(), "b");
    assert_eq!(h.get_items().iter().map(|s| s.to_string()).collect::<Vec<_>>(), ["rust", "b"]);
    let removes: Vec<_> = ElementHandle::find_by_accessible_label(&h, "Remove").filter(|e| e.accessible_role() == Some(AccessibleRole::Button)).collect();
    // One for the pair, two for the items.
    assert_eq!(removes.len(), 3);
    removes[1].invoke_accessible_default_action();
    assert_eq!(h.get_items().iter().map(|s| s.to_string()).collect::<Vec<_>>(), ["b"]);
    let fields: Vec<_> = ElementHandle::find_by_accessible_label(&h, "Tag").filter(|e| e.accessible_role() == Some(AccessibleRole::TextInput)).collect();
    assert_eq!(fields.len(), 1);
    assert_eq!(fields[0].accessible_value().unwrap_or_default(), "b", "the rows rebuild from the new list");
}

cases!(rows: adding_a_pair_focuses_its_key, editing_a_value_reports_every_pair, removing_rows);
