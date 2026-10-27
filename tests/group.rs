// InputGroup and InputAddon, after ely-core/src/forms/group.rs.
#[macro_use]
mod common;
use common::*;
use i_slint_backend_testing::{AccessibleRole, ElementHandle};
use slint::ComponentHandle;

slint::slint! {
    import { InputGroup, InputAddon, Theme, ControlSize, TextFields, TextFit } from "ui/index.slint";
    export { TextFields, TextFit }
    export component Harness inherits Window {
        width: 400px;
        height: 100px;
        in-out property <string> site;
        out property <bool> focused: group.has-focus;
        out property <length> group-height: group.height;
        out property <length> pad: Theme.control-padding(ControlSize.md);
        group := InputGroup {
            x: 0;
            y: 0;
            width: 300px;
            text <=> root.site;
            before: ["https://"];
            after: [".com"];
        }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    ely_slint::wire_fields!(h);
    show(&h);
    h
}

fn text(h: &Harness, label: &str) -> ElementHandle {
    ElementHandle::find_by_accessible_label(h, label).find(|e| e.accessible_role() == Some(AccessibleRole::Text)).unwrap()
}

fn a_press_on_an_addon_focuses_the_field() {
    let h = harness();
    let scheme = text(&h, "https://");
    let at = scheme.absolute_position();
    click(h.window(), at.x + 2.0, at.y + 2.0);
    assert!(h.get_focused());
    tap(h.window(), "e");
    assert_eq!(h.get_site(), "e");
}

fn addons_sit_at_the_ends_inside_the_border() {
    let h = harness();
    let pad = h.get_pad();
    let scheme = text(&h, "https://");
    assert_eq!(scheme.absolute_position().x, 1.0 + pad);
    let tld = text(&h, ".com");
    assert_eq!(tld.absolute_position().x + tld.size().width, 300.0 - 1.0 - pad);
    assert_eq!(h.get_group_height(), 28.0);
}

cases!(group: a_press_on_an_addon_focuses_the_field, addons_sit_at_the_ends_inside_the_border);
