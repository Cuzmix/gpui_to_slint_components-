// IconPicker and EmojiPicker, after ely-core/src/forms/glyphs.rs and picks.rs.
#[macro_use]
mod common;
use common::*;
use i_slint_backend_testing::{AccessibleRole, ElementHandle};
use slint::ComponentHandle;
use slint::platform::Key;

slint::slint! {
    import { IconPicker, EmojiPicker, Glyphs, GlyphLayout, GlyphRow, IconEntry, TextFields, TextFit } from "ui/index.slint";
    export { Glyphs, GlyphLayout, GlyphRow, IconEntry, TextFields, TextFit }
    export component Harness inherits Window {
        width: 760px;
        height: 420px;
        in-out property <string> chosen-icon: "star";
        in-out property <string> emoji;
        in-out property <int> changes;
        out property <int> icon-cursor: icons.cursor;
        out property <int> emoji-cursor: emoji.cursor;
        public function focus-icons() { icons.focus(); }
        public function focus-emoji() { emoji.focus(); }
        icons := IconPicker {
            x: 10px; y: 10px;
            selected <=> root.chosen-icon;
            selection-changed => { root.changes += 1; }
        }
        emoji := EmojiPicker {
            x: 390px; y: 10px;
            selected <=> root.emoji;
            selection-changed => { root.changes += 1; }
        }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    ely_slint::wire_fields!(h);
    ely_slint::wire_glyphs!(h);
    show(&h);
    h
}

fn type_in(h: &Harness, text: &str) {
    for ch in text.chars() {
        tap(h.window(), ch.to_string());
    }
    advance(0);
}

fn cell(h: &Harness, name: &str) -> ElementHandle {
    ElementHandle::find_by_accessible_label(h, name).find(|e| e.accessible_role() == Some(AccessibleRole::ListItem)).unwrap_or_else(|| panic!("no cell {name}"))
}

fn icons_are_found_by_name_and_picked_with_enter() {
    let h = harness();
    h.invoke_focus_icons();
    type_in(&h, "star");
    assert_eq!(cell(&h, "star").accessible_item_selected(), Some(true));
    for _ in 0.."star".len() {
        tap(h.window(), Key::Backspace);
    }
    type_in(&h, "arrow up");
    assert!(ElementHandle::find_by_accessible_label(&h, "arrow-up").count() > 0);
    assert!(ElementHandle::find_by_accessible_label(&h, "star").next().is_none());
    tap(h.window(), Key::Return);
    advance(0);
    assert_eq!(h.get_chosen_icon(), "a-arrow-up", "the first match");
    assert_eq!(h.get_changes(), 1);
}

fn nothing_found_says_so() {
    let h = harness();
    h.invoke_focus_icons();
    type_in(&h, "zzzz");
    assert_eq!(ElementHandle::find_by_accessible_label(&h, "No icons match").count(), 1);
    tap(h.window(), Key::Return);
    advance(0);
    assert_eq!(h.get_changes(), 0);
}

fn arrows_walk_the_grid_and_a_new_search_resets_the_cursor() {
    let h = harness();
    h.invoke_focus_emoji();
    tap(h.window(), Key::DownArrow);
    advance(0);
    assert_eq!(h.get_emoji_cursor(), 0);
    tap(h.window(), Key::RightArrow);
    tap(h.window(), Key::DownArrow);
    advance(0);
    assert_eq!(h.get_emoji_cursor(), 9);
    // Far down, the cursor's row is scrolled into view and drawn.
    for _ in 0..20 {
        tap(h.window(), Key::DownArrow);
    }
    advance(0);
    assert_eq!(h.get_emoji_cursor(), 9 + 20 * 8);
    let name = ElementHandle::find_by_element_type_name(&h, "FocusScope").find(|e| e.accessible_role() == Some(AccessibleRole::Table) && e.accessible_label().as_deref() == Some("Search emoji")).unwrap().accessible_value().unwrap();
    assert!(ElementHandle::find_by_accessible_label(&h, &name).count() > 0, "{name} is drawn");
    tap(h.window(), " ");
    advance(0);
    assert!(!h.get_emoji().is_empty());
    assert_eq!(h.get_changes(), 1);
    h.invoke_focus_emoji();
    type_in(&h, "sparkles");
    assert_eq!(h.get_emoji_cursor(), 0);
    tap(h.window(), Key::Return);
    advance(0);
    assert_eq!(h.get_emoji(), "✨");
}

fn emoji_sit_under_their_groups_until_searched() {
    let h = harness();
    assert_eq!(ElementHandle::find_by_accessible_label(&h, "Smileys & Emotion").count(), 1);
    h.invoke_focus_emoji();
    type_in(&h, "tada");
    assert_eq!(ElementHandle::find_by_accessible_label(&h, "Smileys & Emotion").count(), 0);
    cell(&h, "party popper").invoke_accessible_default_action();
    advance(0);
    assert_eq!(h.get_emoji(), "🎉");
}

cases!(glyph_pickers: icons_are_found_by_name_and_picked_with_enter, nothing_found_says_so, arrows_walk_the_grid_and_a_new_search_resets_the_cursor, emoji_sit_under_their_groups_until_searched);
