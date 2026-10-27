// MentionInput, after ely-core/src/forms/mention.rs.
#[macro_use]
mod common;
use common::*;
use slint::ComponentHandle;
use slint::platform::Key;

slint::slint! {
    import { MentionInput, Mentions, MentionTrigger, MentionAt, Choice, Choices, OptionFloat, OptionLayer, TextFields, TextFit } from "ui/index.slint";
    export { Mentions, MentionTrigger, MentionAt, Choice, Choices, TextFields, TextFit }
    export component Harness inherits Window {
        width: 500px;
        height: 300px;
        in-out property <string> reply;
        out property <bool> open: m.open;
        out property <int> highlighted: m.highlighted;
        out property <int> rows: OptionFloat.rows.length;
        out property <string> first: OptionFloat.rows.length > 0 ? OptionFloat.rows[0].label : "";
        out property <length> list-x: OptionFloat.anchor-x;
        public function focus-m() { m.focus(); }
        m := MentionInput {
            x: 10px;
            y: 10px;
            width: 400px;
            text <=> root.reply;
            label: "Reply";
            triggers: [
                { mark: "@", handles: ["ada", "adam", "bo"] },
                { mark: "#", handles: ["release", "review"] },
            ];
        }
        OptionLayer { }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    ely_slint::wire_fields!(h);
    ely_slint::wire_options!(h);
    ely_slint::wire_mentions!(h);
    show(&h);
    h
}

fn typing(h: &Harness, text: &str) {
    for ch in text.chars() {
        tap(h.window(), ch.to_string());
    }
    advance(0);
}

fn a_trigger_offers_matching_handles() {
    let h = harness();
    h.invoke_focus_m();
    typing(&h, "hi @ad");
    assert!(h.get_open());
    assert_eq!(h.get_rows(), 2);
    assert_eq!(h.get_first(), "@ada");
    assert!(h.get_list_x() > 20.0, "the list sits under the trigger, not the field's start");
    typing(&h, " ");
    assert!(!h.get_open(), "a space ends the word");
}

fn mid_word_is_no_trigger() {
    let h = harness();
    h.invoke_focus_m();
    typing(&h, "mail@ad");
    assert!(!h.get_open());
}

fn arrows_choose_and_enter_puts_it_in() {
    let h = harness();
    h.invoke_focus_m();
    typing(&h, "see #re");
    tap(h.window(), Key::DownArrow);
    assert_eq!(h.get_highlighted(), 1);
    tap(h.window(), Key::Return);
    advance(0);
    assert_eq!(h.get_reply(), "see #review ");
    assert!(!h.get_open());
    typing(&h, "ok");
    assert_eq!(h.get_reply(), "see #review ok", "the caret sits after the space");
}

fn escape_hides_until_the_trigger_ends() {
    let h = harness();
    h.invoke_focus_m();
    typing(&h, "@a");
    tap(h.window(), Key::Escape);
    advance(0);
    assert!(!h.get_open());
    typing(&h, "d");
    assert!(!h.get_open(), "still the same trigger");
    typing(&h, " @b");
    assert!(h.get_open(), "a new trigger offers again");
}

cases!(mention_input: a_trigger_offers_matching_handles, mid_word_is_no_trigger, arrows_choose_and_enter_puts_it_in, escape_hides_until_the_trigger_ends);
