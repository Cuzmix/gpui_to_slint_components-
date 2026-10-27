// SearchInput and the floating option list, after ely-core/src/forms/{search,options}.rs.
#[macro_use]
mod common;
use common::*;
use i_slint_backend_testing::{AccessibleRole, ElementHandle};
use slint::ComponentHandle;

slint::slint! {
    import { SearchInput, OptionLayer, TextFields, TextFit, Choices, Choice, Toggles, Theme } from "ui/index.slint";
    export { TextFields, TextFit, Choices, Choice, Toggles }
    export component Harness inherits Window {
        width: 400px;
        height: 400px;
        in-out property <string> query;
        in-out property <string> last-pick;
        in-out property <length> field-y: 20px;
        out property <bool> shown: layer.shown;
        out property <bool> up: layer.up;
        out property <float> entrance: layer.entrance;
        out property <bool> focused: search.has-focus;
        public function focus-search() { search.focus(); }
        search := SearchInput {
            x: 20px;
            y: root.field-y;
            width: 200px;
            text <=> root.query;
            placeholder: "Search";
            history: ["invoice", "receipt"];
            picked(query) => { root.last-pick = query; }
        }
        layer := OptionLayer { }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    ely_slint::wire_fields!(h);
    ely_slint::wire_options!(h);
    ely_slint::wire_toggles!(h);
    show(&h);
    h
}

fn rows(h: &Harness) -> Vec<ElementHandle> {
    ElementHandle::find_by_accessible_label(h, "invoice")
        .chain(ElementHandle::find_by_accessible_label(h, "receipt"))
        .filter(|e| e.accessible_role() == Some(AccessibleRole::ListItem))
        .collect()
}

fn recent_searches_show_while_focused_and_empty() {
    let h = harness();
    assert!(!h.get_shown());
    h.invoke_focus_search();
    advance(0);
    assert!(h.get_shown());
    assert_eq!(rows(&h).len(), 2);
    tap(h.window(), "x");
    advance(0);
    assert!(!h.get_shown(), "typing hides the list");
}

fn a_pick_fills_the_field_and_keeps_focus() {
    let h = harness();
    h.invoke_focus_search();
    advance(0);
    let receipt = rows(&h).into_iter().nth(1).unwrap();
    let at = receipt.absolute_position();
    let size = receipt.size();
    click(h.window(), at.x + size.width / 2.0, at.y + size.height / 2.0);
    assert_eq!(h.get_query(), "receipt");
    assert_eq!(h.get_last_pick(), "receipt");
    assert!(h.get_focused(), "a pick leaves focus in the field");
    advance(0);
    assert!(!h.get_shown());
}

fn it_opens_below_and_rises_in() {
    let h = harness();
    h.invoke_focus_search();
    advance(0);
    assert!(!h.get_up());
    assert!(h.get_entrance() < 1.0);
    advance(200);
    assert_eq!(h.get_entrance(), 1.0);
    assert!(!h.window().has_active_animations(), "nothing ticks once it has risen");
    let first = rows(&h).into_iter().next().unwrap();
    // Under the field (20 + 28) with a 4 px gap, then the hairline and 4 px inside.
    assert_eq!(first.absolute_position().y, 48.0 + 4.0 + 5.0);
    assert_eq!(first.absolute_position().x, 20.0 + 5.0);
}

fn it_opens_over_the_field_without_room_below() {
    let h = harness();
    h.set_field_y(340.0);
    h.invoke_focus_search();
    advance(0);
    advance(200);
    assert!(h.get_up());
    let last = rows(&h).into_iter().nth(1).unwrap();
    // The list ends 4 px over the field, inside its hairline and 4 px padding.
    assert_eq!(last.absolute_position().y + last.size().height, 340.0 - 4.0 - 5.0);
}

cases!(search: recent_searches_show_while_focused_and_empty, a_pick_fills_the_field_and_keeps_focus, it_opens_below_and_rises_in, it_opens_over_the_field_without_room_below);
