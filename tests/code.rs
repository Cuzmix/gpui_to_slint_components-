// CodeInput and JsonInput, after ely-core/src/forms/code.rs.
#[macro_use]
mod common;
use common::*;
use i_slint_backend_testing::ElementHandle;
use slint::ComponentHandle;

slint::slint! {
    import { CodeInput, JsonInput, TextFields, TextFit } from "ui/index.slint";
    export { TextFields, TextFit }
    export component Harness inherits Window {
        width: 500px;
        height: 300px;
        in-out property <string> code: "let x = f(1)";
        in-out property <string> json;
        out property <string> error: json-field.error;
        CodeInput {
            x: 10px; y: 10px; width: 420px;
            label: "Query";
            text <=> root.code;
        }
        json-field := JsonInput {
            x: 10px; y: 60px; width: 420px;
            label: "Config";
            rows-min: 4;
            rows-max: 8;
            text <=> root.json;
        }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    ely_slint::wire_fields!(h);
    show(&h);
    h
}

fn code_reads_whole_in_its_field() {
    let h = harness();
    assert_eq!(ElementHandle::find_by_accessible_label(&h, "Query").next().unwrap().accessible_value().unwrap(), "let x = f(1)");
}

fn json_errors_show_under_the_field() {
    let h = harness();
    assert_eq!(h.get_error(), "");
    h.set_json("{\"a\": ".into());
    assert_eq!(h.get_error(), "EOF while parsing a value at line 1 column 6");
    assert_eq!(ElementHandle::find_by_accessible_label(&h, "EOF while parsing a value at line 1 column 6").count(), 1);
    h.set_json("{\"a\": [1, true]}".into());
    assert_eq!(h.get_error(), "");
    h.set_json("   ".into());
    assert_eq!(h.get_error(), "", "blank reads as nothing to check");
}

cases!(code: code_reads_whole_in_its_field, json_errors_show_under_the_field);
