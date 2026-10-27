// SegmentedControl, after ely-core/src/buttons/segmented.rs.
#[macro_use]
mod common;
use common::*;
use i_slint_backend_testing::{AccessibleRole, ElementHandle};
use slint::ComponentHandle;
use slint::platform::Key;

slint::slint! {
    import { SegmentedControl, Segment, Icons, TextFormat } from "ui/index.slint";
    export { TextFormat }
    export component Harness inherits Window {
        width: 600px;
        height: 300px;
        in-out property <string> view: "list";
        in-out property <length> strip-width: 360px;
        in-out property <length> icons-width: 400px;
        in-out property <string> heard;
        in-out property <int> changes;
        out property <bool> compact: icons.compact;
        out property <length> scrolled: words.scrolled;
        VerticalLayout {
            alignment: start;
            spacing: 8px;
            HorizontalLayout {
                alignment: start;
                words := SegmentedControl {
                    width: root.strip-width;
                    segments: [
                        { value: "list", label: "List" },
                        { value: "board", label: "Board" },
                        { value: "timeline", label: "Timeline" },
                    ];
                    selected <=> root.view;
                    selection-changed(value) => {
                        root.heard = value;
                        root.changes += 1;
                    }
                }
            }
            HorizontalLayout {
                alignment: start;
                icons := SegmentedControl {
                    width: root.icons-width;
                    segments: [
                        { value: "day", label: "Day", icon: Icons.sun },
                        { value: "night", label: "Night", icon: Icons.moon },
                    ];
                    selected: "day";
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Box_ {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

fn boxed(e: &ElementHandle) -> Box_ {
    let (at, size) = (e.absolute_position(), e.size());
    Box_ { x: at.x, y: at.y, w: size.width, h: size.height }
}

fn segment(h: &Harness, label: &str) -> ElementHandle {
    let found: Vec<_> = ElementHandle::find_by_accessible_label(h, label)
        .filter(|e| e.accessible_role() == Some(AccessibleRole::RadioButton))
        .collect();
    assert_eq!(found.len(), 1, "one segment named {label}");
    found.into_iter().next().unwrap()
}

// The first control's thumb (the second's comes later in the tree).
fn thumb(h: &Harness) -> Box_ {
    boxed(&ElementHandle::find_by_element_id(h, "ThumbCell::thumb").next().expect("a thumb"))
}

fn press(h: &Harness, label: &str) {
    let b = boxed(&segment(h, label));
    click(h.window(), b.x + b.w / 2.0, b.y + b.h / 2.0);
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    ely_slint::wire_text_format!(h);
    show(&h);
    advance(16);
    h
}

// Segments fill the strip inside its 2 px padding and hairline, all grown by the same amount;
// each reads as a radio button, the chosen one checked; the thumb sits under it.
fn segments_fill_the_strip_and_the_thumb_marks_the_choice() {
    let h = harness();
    let (list, board, timeline) = (boxed(&segment(&h, "List")), boxed(&segment(&h, "Board")), boxed(&segment(&h, "Timeline")));
    assert_eq!(list.x, 3.0);
    assert_eq!(board.x, list.x + list.w);
    assert_eq!(timeline.x + timeline.w, 360.0 - 3.0);
    assert_eq!((list.h, list.y), (28.0, 3.0));
    assert!(timeline.w > board.w && board.w > list.w, "each keeps its content's width");
    assert_eq!(segment(&h, "List").accessible_checked(), Some(true));
    assert_eq!(segment(&h, "Board").accessible_checked(), Some(false));
    let t = thumb(&h);
    assert_eq!((t.x, t.w, t.y, t.h), (list.x, list.w, list.y, list.h));
}

// A press on another segment chooses it and reports it once; the thumb springs over SLOW,
// overshooting a little; a press on the chosen one does nothing.
fn a_press_chooses_and_the_thumb_springs_over() {
    let h = harness();
    let (list, timeline) = (boxed(&segment(&h, "List")), boxed(&segment(&h, "Timeline")));
    press(&h, "Timeline");
    assert_eq!(h.get_view(), "timeline");
    assert_eq!((h.get_heard().as_str(), h.get_changes()), ("timeline", 1));
    advance(0);
    let start = thumb(&h);
    assert!((start.x - list.x).abs() < 0.5, "leaves from the old choice: {start:?}");
    let mut peak = 0f32;
    for _ in 0..20 {
        advance(16);
        peak = peak.max(thumb(&h).x);
    }
    assert!(peak > timeline.x, "overshoots: {peak} past {}", timeline.x);
    advance(100);
    let (end, timeline) = (thumb(&h), boxed(&segment(&h, "Timeline")));
    assert_eq!((end.x, end.w), (timeline.x, timeline.w), "settles on the choice, now in semibold");
    press(&h, "Timeline");
    assert_eq!(h.get_changes(), 1);
}

// Tab reaches each segment; Enter chooses on release; a press takes no focus.
fn keys_choose() {
    let h = harness();
    tab(h.window());
    tab(h.window());
    key_down(h.window(), Key::Return);
    assert_eq!(h.get_view(), "list");
    key_up(h.window(), Key::Return);
    assert_eq!(h.get_view(), "board");
}

// Too narrow for every word, all-icon segments show icons only from the next frame, each word a
// tooltip; narrower than two controls a segment in all (one when compact), the strip scrolls and Tab
// brings the focused segment into view.
fn narrow_strips_go_compact_then_scroll() {
    let h = harness();
    assert!(!h.get_compact());
    h.set_icons_width(80.0);
    advance(16);
    assert!(h.get_compact());
    assert!(ElementHandle::find_by_accessible_label(&h, "Day").any(|e| e.accessible_role() == Some(AccessibleRole::RadioButton)));

    h.set_strip_width(120.0);
    advance(16);
    let (list, timeline) = (boxed(&segment(&h, "List")), boxed(&segment(&h, "Timeline")));
    assert_eq!(timeline.x + timeline.w - list.x, 3.0 * 56.0, "two controls a segment in all");
    assert_eq!(h.get_scrolled(), 0.0);
    tab(h.window());
    tab(h.window());
    tab(h.window());
    advance(16);
    assert_eq!(h.get_scrolled(), 3.0 * 56.0 - (120.0 - 6.0), "the last segment's end in view");
}

cases!(segmented: segments_fill_the_strip_and_the_thumb_marks_the_choice, a_press_chooses_and_the_thumb_springs_over, keys_choose, narrow_strips_go_compact_then_scroll);
