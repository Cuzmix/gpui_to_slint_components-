#[macro_use]
mod common;
use common::*;
use slint::ComponentHandle;
use slint::platform::Key;

slint::slint! {
    import { TextFormat, PlatformStyle, RollCell, Theme, CopyButton, CopyableText, Emoji, SelectableText, TextSize } from "ui/index.slint";
    export { TextFormat, PlatformStyle, RollCell }
    export component Harness inherits Window {
        width: 400px;
        height: 300px;
        in-out property <string> glyph;
        in-out property <string> shortcode: "tada";
        in-out property <string> selectable: "error: the build failed";
        in-out property <int> copies;
        out property <bool> copied: copy.copied;
        out property <length> copy-w: copy.width;
        out property <string> emoji: e.text;
        out property <bool> emoji-known: e.known;
        out property <length> emoji-size: e.font-size;
        out property <int> anchor: sel.anchor;
        out property <int> cursor: sel.cursor;
        out property <bool> sel-focused: sel.has-focus;
        out property <length> sel-h: sel.height;
        copy := CopyButton { x: 0; y: 0; text: "sk-live-4f2a"; did-copy => { root.copies += 1; } }
        CopyableText { x: 0; y: 40px; text: "api key"; mono: true; }
        e := Emoji { x: 0; y: 80px; glyph: root.glyph; shortcode: root.shortcode; }
        sel := SelectableText { x: 0; y: 120px; width: 400px; text: root.selectable; }
    }
}

fn wired() -> Harness {
    let h = Harness::new().unwrap();
    ely_slint::wire_text_format!(h);
    show(&h);
    h
}

fn a_copy_shows_the_check_for_a_moment() {
    let h = wired();
    assert_eq!(h.get_copy_w(), 24.0);
    click(h.window(), 12.0, 12.0);
    assert_eq!(h.get_copies(), 1);
    assert!(h.get_copied());
    click(h.window(), 12.0, 12.0);
    assert_eq!(h.get_copies(), 1, "the check takes no press");
    advance(1399);
    assert!(h.get_copied());
    advance(2);
    assert!(!h.get_copied());
}

fn emoji_resolves_shortcodes_or_shows_nothing() {
    let h = wired();
    assert_eq!(h.get_emoji(), "🎉");
    assert_eq!(h.get_emoji_size(), 16.0);
    h.set_shortcode("not_an_emoji_name".into());
    assert!(!h.get_emoji_known());
    h.set_glyph("✨".into());
    assert_eq!(h.get_emoji(), "✨");
}

fn a_drag_selects_and_a_press_focuses() {
    let h = wired();
    mouse_move(h.window(), 1.0, 130.0);
    mouse_down(h.window(), 1.0, 130.0);
    assert!(h.get_sel_focused());
    mouse_move(h.window(), 60.0, 130.0);
    mouse_up(h.window(), 60.0, 130.0);
    assert_eq!(h.get_anchor(), 0);
    assert!(h.get_cursor() > 0);
}

fn select_all_and_moving_keys() {
    let h = wired();
    click(h.window(), 5.0, 130.0);
    let secondary = if cfg!(target_os = "macos") { Key::Meta } else { Key::Control };
    key_down(h.window(), secondary);
    tap(h.window(), "a");
    key_up(h.window(), secondary);
    assert_eq!((h.get_anchor(), h.get_cursor()), (0, 23));
    tap(h.window(), Key::LeftArrow);
    assert_eq!((h.get_anchor(), h.get_cursor()), (0, 23), "arrows move nothing");
    h.set_selectable("new".into());
    advance(16);
    assert_eq!((h.get_anchor(), h.get_cursor()), (0, 0), "a new text clears the selection");
}

fn selectable_text_takes_the_ambient_leading() {
    let h = wired();
    assert_eq!(h.get_sel_h(), 21.0);
}

cases!(copy_select: a_copy_shows_the_check_for_a_moment, emoji_resolves_shortcodes_or_shows_nothing, a_drag_selects_and_a_press_focuses, select_all_and_moving_keys, selectable_text_takes_the_ambient_leading);
