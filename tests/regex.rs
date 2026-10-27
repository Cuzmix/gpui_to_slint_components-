// RegexInput, after ely-core/src/forms/pattern.rs; and the colored text layer (G53).
#[macro_use]
mod common;
use common::*;
use slint::ComponentHandle;

slint::slint! {
    import { RegexInput, TextFields, TextFit } from "ui/index.slint";
    export { TextFields, TextFit }
    export component Harness inherits Window {
        width: 500px;
        height: 200px;
        in-out property <string> pattern: "a+";
        out property <string> error: re.error;
        out property <bool> painting: probe.painting;
        public function focus-re() { re.focus(); }
        re := RegexInput {
            x: 10px;
            y: 10px;
            width: 300px;
            text <=> root.pattern;
            label: "Pattern";
        }
        // The layer itself, in a narrow field.
        probe := TextField {
            x: 10px;
            y: 120px;
            width: 80px;
            text: root.pattern;
            highlighting: true;
        }
    }
    import { TextField } from "ui/components/text-field.slint";
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    ely_slint::wire_fields!(h);
    show(&h);
    h
}

fn a_pattern_that_compiles_has_no_error() {
    let h = harness();
    assert_eq!(h.get_error(), "");
    h.set_pattern("".into());
    assert_eq!(h.get_error(), "", "empty is no error");
}

fn typing_checks_as_you_type() {
    let h = harness();
    h.invoke_focus_re();
    tap(h.window(), slint::platform::Key::End);
    tap(h.window(), "(");
    assert_eq!(h.get_pattern(), "a+(");
    assert_eq!(h.get_error(), "error: unclosed group");
    tap(h.window(), ")");
    assert_eq!(h.get_error(), "");
}

fn the_layer_draws_only_while_the_text_fits() {
    let h = harness();
    assert!(h.get_painting(), "short text is colored");
    h.set_pattern("a much longer pattern than fits".into());
    assert!(!h.get_painting(), "wider than the field: plain");
    h.set_pattern("".into());
    assert!(!h.get_painting(), "the placeholder shows instead");
}

cases!(regex_input: a_pattern_that_compiles_has_no_error, typing_checks_as_you_type, the_layer_draws_only_while_the_text_fits);
