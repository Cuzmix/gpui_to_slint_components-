// ChoiceChips, after ely-core/src/forms/chips.rs.
#[macro_use]
mod common;
use common::*;
use slint::{ComponentHandle, Model};

slint::slint! {
    import { ChoiceChips, Choice, Choices, Toggles } from "ui/index.slint";
    export { Choice, Choices, Toggles }
    export component Harness inherits Window {
        width: 400px;
        height: 200px;
        in-out property <[string]> size: ["m"];
        in-out property <bool> multiple;
        in-out property <int> changes;
        out property <length> chips-height: chips.preferred-height;
        chips := ChoiceChips {
            x: 10px;
            y: 10px;
            width: 300px;
            multiple: root.multiple;
            choices: [
                { value: "s", label: "S" },
                { value: "m", label: "M" },
                { value: "xl", label: "XL", disabled: true },
                { value: "l", label: "L" },
            ];
            selected <=> root.size;
            selection-changed => { root.changes += 1; }
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

fn size(h: &Harness) -> Vec<String> {
    h.get_size().iter().map(|l| l.to_string()).collect()
}

fn one_chip_replaces() {
    let h = harness();
    // The first chip starts at x 10.
    click(h.window(), 20.0, 22.0);
    assert_eq!(size(&h), ["s"]);
    assert_eq!(h.get_chips_height(), 24.0, "one line of sm chips");
}

fn filter_chips_toggle_and_tab_skips_disabled() {
    let h = harness();
    h.set_multiple(true);
    advance(0);
    tab(h.window());
    tap(h.window(), " ");
    assert_eq!(size(&h), ["s", "m"]);
    tab(h.window());
    tab(h.window());
    // Past disabled XL to L.
    tap(h.window(), slint::platform::Key::Return);
    assert_eq!(size(&h), ["s", "m", "l"]);
    assert_eq!(h.get_changes(), 2);
}

cases!(choice_chips: one_chip_replaces, filter_chips_toggle_and_tab_skips_disabled);
