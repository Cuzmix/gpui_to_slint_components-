// ColumnShell, after ely-core/src/shell/tests.rs (columns) and tests/border.rs.
#[macro_use]
mod common;
use std::cell::Cell;
use std::rc::Rc;

use common::*;
use i_slint_backend_testing::ElementHandle;
use slint::ComponentHandle;

slint::slint! {
    import { ColumnShell, ColumnShells, ShellColumnSpec, ShellRoom, ShellColumnFrame, ShellColumnHeader, ShellColumnTitle, ShellColumnBody, WindowChrome, ChromeStyle, IconButton, Icons } from "ui/index.slint";
    export { ColumnShells, ShellColumnSpec, ShellRoom, ShellColumnFrame, WindowChrome, ChromeStyle }
    export component Harness inherits Window {
        width: 900px;
        height: 400px;
        in property <length> shell-width: 900px;
        in property <ChromeStyle> style: ChromeStyle.windows;
        in-out property <length> w0: 240px;
        in-out property <length> w2: 200px;
        out property <int> closes;
        out property <int> resizes;
        out property <length> c0-x: shell.frames[0].x;
        out property <length> c0-w: shell.frames[0].width;
        out property <length> c1-x: shell.frames[1].x;
        out property <length> c1-w: shell.frames[1].width;
        out property <length> c2-x: shell.frames[2].x;
        out property <length> c2-w: shell.frames[2].width;
        out property <length> body-h: body.height;

        HorizontalLayout {
            alignment: start;
            shell := ColumnShell {
                width: root.shell-width;
                platform: root.style;
                handles-close: true;
                columns: [
                    { resizable: true, min: 180px, max: 320px },
                    { fills: true },
                    { resizable: true, min: 150px, max: 300px },
                ];
                widths: [root.w0, 0, root.w2];
                close-requested => {
                    root.closes += 1;
                }
                resized(index, width) => {
                    root.resizes += 1;
                    if index == 0 {
                        root.w0 = width;
                    } else {
                        root.w2 = width;
                    }
                }
                ShellColumnHeader {
                    column: shell.frames[0];
                    IconButton {
                        icon: Icons.panel-left;
                        icon-name: "side-lead";
                    }
                    ShellColumnTitle { }
                }

                body := ShellColumnBody {
                    column: shell.frames[0];
                }

                ShellColumnHeader {
                    column: shell.frames[1];
                    ShellColumnTitle {
                        text: "Inbox";
                    }
                    IconButton {
                        icon: Icons.info;
                        icon-name: "main-action";
                    }
                }

                ShellColumnHeader {
                    column: shell.frames[2];
                    ShellColumnTitle {
                        text: "Details";
                    }
                    IconButton {
                        icon: Icons.panel-right;
                        icon-name: "last-action";
                    }
                }
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

fn find(h: &Harness, label: &str) -> Vec<Box_> {
    ElementHandle::find_by_accessible_label(h, label)
        .map(|e| {
            let (at, size) = (e.absolute_position(), e.size());
            Box_ { x: at.x, y: at.y, w: size.width, h: size.height }
        })
        .collect()
}

fn one(h: &Harness, label: &str) -> Box_ {
    let mut found = find(h, label);
    assert!(!found.is_empty(), "nothing labelled {label:?}");
    found.remove(0)
}

fn harness() -> (Harness, Rc<Cell<u32>>) {
    let h = Harness::new().unwrap();
    ely_slint::wire_column_shell!(h);
    let chrome = h.global::<WindowChrome>();
    chrome.set_system_lights(false);
    let moves = Rc::new(Cell::new(0));
    let seen = moves.clone();
    chrome.on_start_move(move || seen.set(seen.get() + 1));
    show(&h);
    advance(16);
    (h, moves)
}

// Presses at x, y, goes past the 2 px threshold, then to each offset, and lets go.
fn drag_path(h: &Harness, x: f32, y: f32, path: &[f32]) {
    mouse_move(h.window(), x, y);
    mouse_down(h.window(), x, y);
    mouse_move(h.window(), x + 3.0, y);
    mouse_move(h.window(), x, y);
    for by in path {
        mouse_move(h.window(), x + by, y);
    }
    let end = x + path.last().copied().unwrap_or(0.0);
    mouse_up(h.window(), end, y);
    advance(16);
}

// Set widths hold, the filling column takes the rest, and every column runs the shell's height.
fn columns_keep_their_widths_and_the_fill_takes_the_rest() {
    let (h, _) = harness();
    assert_eq!((h.get_c0_x(), h.get_c0_w()), (0.0, 240.0));
    assert_eq!((h.get_c1_x(), h.get_c1_w()), (240.0, 460.0));
    assert_eq!((h.get_c2_x(), h.get_c2_w()), (700.0, 200.0));
    assert_eq!(h.get_body_h(), 400.0 - 38.0);
}

// Windows' caption buttons end the last header, flush with the shell's edge; Close runs
// `close-requested`.
fn windows_controls_end_the_last_header() {
    let (h, _) = harness();
    let close = one(&h, "Close");
    assert_eq!(close.x + close.w, 900.0);
    assert_eq!(close.h, 37.0);
    // The last header's action ends 8 px before the controls.
    let action = one(&h, "last-action");
    assert_eq!(action.x + action.w, close.x - 46.0 * 2.0 - 8.0);
    click(h.window(), close.x + close.w / 2.0, close.y + close.h / 2.0);
    assert_eq!(h.get_closes(), 1);
}

// Drawn traffic lights lead the first header, inside the sidebar, and its items follow them.
fn drawn_traffic_lights_lead_the_first_header() {
    let (h, _) = harness();
    h.set_style(ChromeStyle::Mac);
    advance(16);
    let close = one(&h, "Close");
    assert_eq!(close.x, 12.0);
    assert!(close.x + 52.0 < 240.0);
    assert_eq!(one(&h, "side-lead").x, 12.0 + 52.0 + 8.0);
}

// Unforced, macOS leaves the first header room for the system's lights; elsewhere the theme's
// controls are drawn.
fn an_unforced_shell_follows_the_system() {
    let (h, _) = harness();
    h.set_style(ChromeStyle::System);
    h.global::<WindowChrome>().set_system_lights(true);
    advance(16);
    assert!(find(&h, "Close").is_empty());
    assert_eq!(one(&h, "side-lead").x, 78.0);
    h.global::<WindowChrome>().set_system_lights(false);
    advance(16);
    assert_eq!(find(&h, "Close").len(), 1);
}

// Short, set widths give way, so the caption buttons stay inside the shell (gpui's test runs
// two columns at 280 px; three need 302 px for their headers).
fn when_short_the_caption_buttons_stay_inside_the_shell() {
    let (h, _) = harness();
    h.set_shell_width(400.0);
    advance(16);
    let close = one(&h, "Close");
    assert!(close.x + close.w <= 400.0, "close ends at {}", close.x + close.w);
    assert!(close.x >= h.get_c2_x(), "the controls sit in the last column");
    assert!(h.get_c0_w() < 240.0 && h.get_c2_w() < 200.0);
}

// A column before the filling one drags by its right border, one after it by its left.
fn a_border_toward_the_filling_column_drags_its_width() {
    let (h, _) = harness();
    drag_path(&h, 237.0, 200.0, &[30.0]);
    assert_eq!(h.get_w0(), 270.0);
    drag_path(&h, 703.0, 200.0, &[-40.0]);
    assert_eq!(h.get_w2(), 240.0);
}

// A drag stops at the column's least and greatest width.
fn a_border_drag_keeps_within_the_limits() {
    let (h, _) = harness();
    drag_path(&h, 237.0, 200.0, &[200.0]);
    assert_eq!(h.get_w0(), 320.0);
    drag_path(&h, 317.0, 200.0, &[-300.0]);
    assert_eq!(h.get_w0(), 180.0);
}

// In a header the border takes the press, so the window does not move.
fn a_border_drag_in_a_header_moves_no_window() {
    let (h, moves) = harness();
    drag_path(&h, 237.0, 19.0, &[20.0]);
    assert_eq!(moves.get(), 0);
    assert_eq!(h.get_w0(), 260.0);
    // The header's empty space does drag it.
    drag_path(&h, 150.0, 19.0, &[20.0]);
    assert_eq!(moves.get(), 1);
}

// A drag back where it set off gives the width the owner had.
fn a_drag_back_where_it_set_off_gives_the_width_it_had() {
    let (h, _) = harness();
    drag_path(&h, 237.0, 200.0, &[40.0, 0.0]);
    assert_eq!(h.get_w0(), 240.0);
}

// A short shell squeezes the columns; a drag moves a border from the owner's width.
fn a_squeezed_column_drags_from_the_owners_width() {
    let (h, _) = harness();
    h.set_shell_width(380.0);
    advance(16);
    let drawn = h.get_c0_w();
    assert!(drawn < 240.0);
    drag_path(&h, drawn - 3.0, 200.0, &[10.0]);
    assert_eq!(h.get_w0(), 250.0);
}

cases!(column_shell: columns_keep_their_widths_and_the_fill_takes_the_rest, windows_controls_end_the_last_header, drawn_traffic_lights_lead_the_first_header, an_unforced_shell_follows_the_system, when_short_the_caption_buttons_stay_inside_the_shell, a_border_toward_the_filling_column_drags_its_width, a_border_drag_keeps_within_the_limits, a_border_drag_in_a_header_moves_no_window, a_drag_back_where_it_set_off_gives_the_width_it_had, a_squeezed_column_drags_from_the_owners_width);
