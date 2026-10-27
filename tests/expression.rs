// ExpressionInput, after ely-core/src/forms/expression.rs.
#[macro_use]
mod common;
use common::*;
use slint::ComponentHandle;

slint::slint! {
    import { ExpressionInput, Formulas, FormulaVariable, FormulaResult, TextFields, TextFit } from "ui/index.slint";
    export { Formulas, FormulaVariable, FormulaResult, TextFields, TextFit }
    export component Harness inherits Window {
        width: 500px;
        height: 200px;
        in-out property <string> formula;
        out property <bool> shown: total.result.shown;
        out property <bool> ok: total.result.ok;
        out property <string> line: total.result.text;
        out property <length> total-height: total.preferred-height;
        public function focus-total() { total.focus(); }
        total := ExpressionInput {
            x: 10px;
            y: 10px;
            width: 300px;
            text <=> root.formula;
            variables: [{ name: "price", value: 12.5 }, { name: "qty", value: 3 }];
            label: "Total";
        }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    ely_slint::wire_fields!(h);
    ely_slint::wire_formulas!(h);
    show(&h);
    h
}

fn blank_shows_no_line() {
    let h = harness();
    assert!(!h.get_shown());
    assert_eq!(h.get_total_height(), 28.0, "the md field alone");
}

fn typing_shows_the_value() {
    let h = harness();
    h.invoke_focus_total();
    for ch in "price * qty".chars() {
        tap(h.window(), ch.to_string());
    }
    assert_eq!(h.get_formula(), "price * qty");
    assert!(h.get_shown() && h.get_ok());
    assert_eq!(h.get_line(), "= 37.5");
    assert!(h.get_total_height() > 28.0, "the line sits under the field");
}

fn a_problem_is_named() {
    let h = harness();
    h.set_formula("tax * 2".into());
    assert!(h.get_shown() && !h.get_ok());
    assert_eq!(h.get_line(), "tax is not defined");
}

cases!(expression_input: blank_shows_no_line, typing_shows_the_value, a_problem_is_named);
