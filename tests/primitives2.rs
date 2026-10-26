#[macro_use]
mod common;
use common::*;

slint::slint! {
    import { Disclosure, Picture, Backdrop, Place, Severity, Severities, Theme, TextSize, Icons, FocusRing, DragGhost, VisuallyHidden } from "ui/index.slint";
    export component Harness inherits Window {
        width: 400px;
        height: 300px;
        in-out property <bool> open: false;
        in-out property <bool> pic-has: true;
        in-out property <bool> pic-loading: false;
        in-out property <bool> show-backdrop: false;
        in-out property <Place> place: center;
        in-out property <int> dismissed;
        in-out property <int> panel-clicks;
        out property <angle> turn: d.turn;
        out property <bool> failed: pic.failed;
        out property <color> warn: Severities.color(Severity.warning);
        out property <color> warn-subtle: Severities.subtle(Severity.warning);
        out property <color> c-warning: Theme.colors.warning;
        out property <color> c-warning-subtle: Theme.colors.warning-subtle;
        in-out property <bool> ring-focused: false;
        out property <color> ring-border: ring.border-color;
        out property <color> c-focus: Theme.colors.focus;
        out property <color> c-border: Theme.colors.border;
        out property <color> c-overlay: Theme.colors.overlay;
        out property <color> ghost-fill: ghost.background;
        out property <color> ghost-border: ghost.border-color;
        out property <length> ghost-h: ghost.height;
        out property <length> sm-line: ghost-text.preferred-height;
        ring := FocusRing { x: 100px; y: 0; width: 20px; height: 20px; focused: ring-focused; rest-color: Theme.colors.border; }
        ghost := DragGhost { x: 150px; y: 0; title: "main.rs"; icon: Icons.file; }
        ghost-text := Text { visible: false; text: "main.rs"; font-size: Theme.text-size(TextSize.sm); font-family: Theme.font-family; }
        hidden := VisuallyHidden { x: 300px; y: 0; text: "3 unread messages"; }
        d := Disclosure { x: 0; y: 0; open: open; }
        pic := Picture { x: 50px; y: 0; width: 40px; height: 40px; has-source: pic-has; loading: pic-loading; source: @image-url("ui/icons/check.svg"); }
        if show-backdrop: Backdrop {
            place: place;
            dismiss => { dismissed += 1; }
            TouchArea {
                width: 100px;
                height: 80px;
                clicked => { panel-clicks += 1; }
            }
        }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    show(&h);
    h
}

fn disclosure_is_still_on_first_paint_and_turns_in_120ms() {
    let h = Harness::new().unwrap();
    h.set_open(true);
    show(&h);
    assert_eq!(h.get_turn(), 90.0);
    h.set_open(false);
    advance(1);
    advance(60);
    let mid = h.get_turn();
    // ease-out-cubic at 0.5 is 0.875 of the way: 90 - 78.75 = 11.25.
    assert!((mid - 11.25).abs() < 3.0, "mid-turn {mid}");
    advance(100);
    assert_eq!(h.get_turn(), 0.0);
}

fn picture_states() {
    let h = harness();
    assert!(!h.get_failed());
    h.set_pic_has(false);
    assert!(!h.get_failed());
}

fn severity_maps_to_palette() {
    let h = harness();
    assert_eq!(h.get_warn(), h.get_c_warning());
    assert_eq!(h.get_warn_subtle(), h.get_c_warning_subtle());
}

fn backdrop_dismisses_outside_children_only() {
    let h = harness();
    h.set_show_backdrop(true);
    advance(1);
    // Centered 100 x 80 panel at (150, 110).
    click(h.window(), 200.0, 150.0);
    assert_eq!((h.get_panel_clicks(), h.get_dismissed()), (1, 0));
    click(h.window(), 10.0, 10.0);
    assert_eq!(h.get_dismissed(), 1);
}

fn backdrop_right_edge_stretches_full_height() {
    let h = harness();
    h.set_place(Place::Right);
    h.set_show_backdrop(true);
    advance(1);
    // The strip along the edge occludes the scrim below the 80 px child, as GPUI's wrapper does.
    click(h.window(), 350.0, 290.0);
    assert_eq!((h.get_panel_clicks(), h.get_dismissed()), (0, 0));
    click(h.window(), 350.0, 40.0);
    assert_eq!(h.get_panel_clicks(), 1);
    click(h.window(), 250.0, 290.0);
    assert_eq!(h.get_dismissed(), 1);
}

fn focus_ring_turns_focus_colored() {
    let h = harness();
    assert_eq!(h.get_ring_border(), h.get_c_border());
    h.set_ring_focused(true);
    assert_eq!(h.get_ring_border(), h.get_c_focus());
}

// px_2 py_1, overlay fill, 1 px border: one sm line plus 4 px above and below.
fn drag_ghost_box() {
    let h = harness();
    assert_eq!(h.get_ghost_fill(), h.get_c_overlay());
    assert_eq!(h.get_ghost_border(), h.get_c_border());
    assert_eq!(h.get_ghost_h(), h.get_sm_line() + 8.0);
}

fn visually_hidden_reads_but_draws_nothing() {
    let h = harness();
    let found: Vec<_> = i_slint_backend_testing::ElementHandle::find_by_accessible_label(&h, "3 unread messages").collect();
    assert!(!found.is_empty());
    for e in found {
        assert_eq!(e.accessible_role(), Some(i_slint_backend_testing::AccessibleRole::Text));
        assert_eq!(e.size().width, 0.0);
    }
}

cases!(primitives2: disclosure_is_still_on_first_paint_and_turns_in_120ms, picture_states, severity_maps_to_palette, backdrop_dismisses_outside_children_only, backdrop_right_edge_stretches_full_height, focus_ring_turns_focus_colored, drag_ghost_box, visually_hidden_reads_but_draws_nothing);
