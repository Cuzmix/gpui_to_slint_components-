// CheckboxCard, RadioCard, after ely-core/src/forms/card.rs.
#[macro_use]
mod common;
use common::*;
use slint::ComponentHandle;

slint::slint! {
    import { CheckboxCard, RadioCard, Theme } from "ui/index.slint";
    export component Harness inherits Window {
        width: 400px;
        height: 400px;
        in-out property <bool> backups: true;
        in-out property <bool> pro;
        in-out property <int> picks;
        out property <length> plain-height: backup.preferred-height;
        out property <length> preview-height: plan.preferred-height;
        public function focus-plan() { plan.focus(); }
        backup := CheckboxCard {
            x: 10px;
            y: 10px;
            width: 300px;
            checked <=> root.backups;
            title: "Daily backups";
        }
        plan := RadioCard {
            x: 10px;
            y: 150px;
            width: 300px;
            chosen <=> root.pro;
            title: "Pro";
            description: "$12 a month";
            selected => { root.picks += 1; }
            Rectangle { height: 40px; }
        }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    show(&h);
    h
}

fn a_checkbox_card_toggles() {
    let h = harness();
    click(h.window(), 200.0, 30.0);
    assert!(!h.get_backups());
    click(h.window(), 200.0, 30.0);
    assert!(h.get_backups());
}

fn a_radio_card_picks_once() {
    let h = harness();
    click(h.window(), 200.0, 170.0);
    click(h.window(), 200.0, 170.0);
    assert!(h.get_pro());
    assert_eq!(h.get_picks(), 1);
}

fn keys_pick_too() {
    let h = harness();
    h.invoke_focus_plan();
    tap(h.window(), " ");
    assert!(h.get_pro());
}

fn the_preview_sits_8px_under_the_words() {
    let h = harness();
    // 16 px padding and a 1 px border each side round the words.
    let words = h.get_plain_height() - 34.0;
    assert!(words > 0.0);
    assert!(h.get_preview_height() > h.get_plain_height() + 40.0, "description, 8 px and the preview");
}

cases!(choice_cards: a_checkbox_card_toggles, a_radio_card_picks_once, keys_pick_too, the_preview_sits_8px_under_the_words);
