// QuickActions, after ely-core/src/buttons/quick.rs.
#[macro_use]
mod common;
use common::*;
use i_slint_backend_testing::{AccessibleRole, ElementHandle};
use slint::ComponentHandle;
use slint::platform::Key;

slint::slint! {
    import { QuickActions, QuickAction, Icons } from "ui/index.slint";
    export component Harness inherits Window {
        width: 400px;
        height: 300px;
        in-out property <int> ran: -1;
        in-out property <int> runs;
        out property <int> selected: quick.selected;
        out property <bool> focused: quick.has-focus;
        quick := QuickActions {
            x: 0;
            y: 0;
            width: 300px;
            actions: [
                { icon: Icons.file-plus, label: "New file", note: "An empty file", shortcut: "⌘N" },
                { icon: Icons.folder-open, label: "Open folder" },
                { icon: Icons.settings, label: "Settings" },
            ];
            activated(ix) => {
                root.ran = ix;
                root.runs += 1;
            }
        }
    }
}

struct Box_ {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

fn row(h: &Harness, label: &str) -> (Box_, bool) {
    let e = ElementHandle::find_by_accessible_label(h, label)
        .find(|e| e.accessible_role() == Some(AccessibleRole::ListItem))
        .unwrap_or_else(|| panic!("a row named {label}"));
    let (at, size) = (e.absolute_position(), e.size());
    (Box_ { x: at.x, y: at.y, w: size.width, h: size.height }, e.accessible_item_selected() == Some(true))
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    show(&h);
    h
}

// Rows sit 4 px inside the hairline, 2 px apart; the first is chosen; a row is its icon box
// (a md control) 6 px from top and bottom.
fn rows_and_their_sizes() {
    let h = harness();
    let (first, chosen) = row(&h, "New file");
    let (second, _) = row(&h, "Open folder");
    assert!(chosen);
    assert_eq!((first.x, first.y, first.w), (5.0, 5.0, 300.0 - 10.0));
    assert_eq!(second.h, 28.0 + 12.0);
    assert_eq!(second.y, first.y + first.h + 2.0);
    assert!(first.h > second.h, "the note adds a line");
}

// Hover chooses; a click runs.
fn the_pointer_chooses_and_runs() {
    let h = harness();
    let (settings, _) = row(&h, "Settings");
    mouse_move(h.window(), settings.x + 20.0, settings.y + 10.0);
    assert_eq!(h.get_selected(), 2);
    assert!(row(&h, "Settings").1);
    click(h.window(), settings.x + 20.0, settings.y + 10.0);
    assert_eq!((h.get_ran(), h.get_runs()), (2, 1));
}

// A Tab stop; Down and Up wrap; Enter runs on the press.
fn arrows_choose_and_enter_runs() {
    let h = harness();
    tab(h.window());
    assert!(h.get_focused());
    tap(h.window(), Key::UpArrow);
    assert_eq!(h.get_selected(), 2, "wraps back");
    tap(h.window(), Key::DownArrow);
    assert_eq!(h.get_selected(), 0, "wraps on");
    tap(h.window(), Key::DownArrow);
    key_down(h.window(), Key::Return);
    assert_eq!((h.get_ran(), h.get_runs()), (1, 1));
}

cases!(quick_actions: rows_and_their_sizes, the_pointer_chooses_and_runs, arrows_choose_and_enter_runs);
