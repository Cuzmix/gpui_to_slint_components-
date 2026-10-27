// Stepper, after ely-core/src/forms/stepper.rs.
#[macro_use]
mod common;
use common::*;
use slint::ComponentHandle;

slint::slint! {
    import { Stepper, TextFormat } from "ui/index.slint";
    export { TextFormat }
    export component Harness inherits Window {
        width: 300px;
        height: 100px;
        in-out property <float> guests: 2;
        in-out property <int> changes;
        out property <length> stepper-height: step.height;
        step := Stepper {
            x: 10px;
            y: 10px;
            minimum: 1;
            maximum: 3;
            value <=> root.guests;
            value-changed => { root.changes += 1; }
        }
        Stepper {
            x: 10px;
            y: 50px;
            value: -1234.5;
            precision: 2;
        }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    ely_slint::wire_text_format!(h);
    show(&h);
    h
}

fn presses_step_within_the_range() {
    let h = harness();
    click_label(&h, "plus");
    assert_eq!(h.get_guests(), 3.0);
    assert_eq!(h.get_changes(), 1);
    click_label(&h, "plus");
    assert_eq!(h.get_guests(), 3.0, "plus is disabled at the top");
    assert_eq!(h.get_changes(), 1);
    click_label(&h, "minus");
    click_label(&h, "minus");
    assert_eq!(h.get_guests(), 1.0);
    click_label(&h, "minus");
    assert_eq!(h.get_guests(), 1.0, "minus is disabled at the bottom");
}

fn it_shows_the_formatted_number() {
    let h = harness();
    assert_eq!(h.get_stepper_height(), 28.0);
    let shown: Vec<String> = i_slint_backend_testing::ElementHandle::find_by_element_type_name(&h, "TypeText")
        .filter_map(|e| e.accessible_label())
        .map(|s| s.to_string())
        .collect();
    assert!(shown.contains(&"2".to_string()), "{shown:?}");
    assert!(shown.contains(&"−1,234.50".to_string()), "{shown:?}");
}

cases!(stepper: presses_step_within_the_range, it_shows_the_formatted_number);
