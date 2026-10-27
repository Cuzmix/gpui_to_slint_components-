// TitleBar, WindowControls, WindowDragArea, ResizeBorder and the window host, after
// ely-core/src/shell/tests.rs, tests/title.rs and frame.rs.
#[macro_use]
mod common;
use std::cell::Cell;
use std::rc::Rc;

use common::*;
use i_slint_backend_testing::ElementHandle;
use slint::ComponentHandle;

slint::slint! {
    import { TitleBar, TitleBarTitle, WindowControls, WindowChrome, ChromeStyle, ResizeEdge, Tiling, ResizeBorder, IconButton, Icons, PlatformStyle } from "ui/index.slint";
    export { WindowChrome, ResizeEdge, Tiling, PlatformStyle, ChromeStyle }
    export component Harness inherits Window {
        width: 600px;
        height: 400px;
        in property <ChromeStyle> style: ChromeStyle.linux;
        in property <PlatformStyle> controls-style: PlatformStyle.windows;
        in property <string> heading: "Ely — Projects";
        in property <length> bar-width: 600px;
        in property <bool> handles: true;
        out property <int> closes;
        out property <int> under-presses;
        out property <int> slot-clicks;

        VerticalLayout {
            alignment: start;
            HorizontalLayout {
                alignment: start;
                bar := TitleBar {
                    width: root.bar-width;
                    platform: root.style;
                    handles-close: root.handles;
                    close-requested => {
                        root.closes += 1;
                    }
                    IconButton {
                        icon: Icons.panel-left;
                        icon-name: "lead";
                        clicked => {
                            root.slot-clicks += 1;
                        }
                    }
                    TitleBarTitle {
                        text: root.heading;
                        bar: bar.frame;
                    }
                }
            }

            Rectangle {
                height: 50px;
                TouchArea {
                    pointer-event(event) => {
                        if event.kind == PointerEventKind.down {
                            root.under-presses += 1;
                        }
                    }
                }
                HorizontalLayout {
                    alignment: start;
                    WindowControls {
                        platform: root.controls-style;
                        handles-close: true;
                    }
                }
            }

            Rectangle {
                height: 200px;
                ResizeBorder { }
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

struct Counts {
    moves: Rc<Cell<u32>>,
    closes: Rc<Cell<u32>>,
}

fn harness() -> (Harness, Counts) {
    let h = Harness::new().unwrap();
    let chrome = h.global::<WindowChrome>();
    chrome.set_system_lights(false);
    let counts = Counts { moves: Rc::default(), closes: Rc::default() };
    let moves = counts.moves.clone();
    chrome.on_start_move(move || moves.set(moves.get() + 1));
    let closes = counts.closes.clone();
    chrome.on_close(move || closes.set(closes.get() + 1));
    show(&h);
    advance(16);
    (h, counts)
}

// Presses at x, y, moves two pixels and one down, releases.
fn wobble(h: &Harness, x: f32, y: f32) {
    mouse_move(h.window(), x, y);
    mouse_down(h.window(), x, y);
    mouse_move(h.window(), x + 2.0, y + 1.0);
    mouse_up(h.window(), x + 2.0, y + 1.0);
    advance(16);
}

// The bar's own Close is its rightmost; the second set below has one too.
fn bar_close(h: &Harness) -> Box_ {
    find(h, "Close").into_iter().min_by(|a, b| a.y.total_cmp(&b.y)).unwrap()
}

// A press that wobbles on Linux's close button still closes; it never moves the window.
fn a_wobbling_press_on_a_linux_button_clicks_it() {
    let (h, counts) = harness();
    let close = bar_close(&h);
    wobble(&h, close.x + close.w / 2.0, close.y + close.h / 2.0);
    assert_eq!(h.get_closes(), 1);
    assert_eq!(counts.moves.get(), 0);
}

// The bar's empty space moves the window, once per press.
fn a_wobbling_press_on_the_bar_moves_the_window() {
    let (h, counts) = harness();
    wobble(&h, 300.0, 19.0);
    assert_eq!(counts.moves.get(), 1);
    // A plain move without a press does nothing.
    mouse_move(h.window(), 310.0, 19.0);
    assert_eq!(counts.moves.get(), 1);
}

// A press on a slot item never moves the window.
fn a_wobbling_press_on_a_slot_moves_no_window() {
    let (h, counts) = harness();
    let lead = one(&h, "lead");
    wobble(&h, lead.x + lead.w / 2.0, lead.y + lead.h / 2.0);
    assert_eq!(counts.moves.get(), 0);
    assert_eq!(h.get_slot_clicks(), 1);
}

// Each style's controls hide what lies under them, gaps included.
fn window_controls_occlude_what_lies_under() {
    let (h, _) = harness();
    for style in [Style::Windows, Style::Mac, Style::Linux] {
        h.set_controls_style(style.into());
        advance(16);
        let before = h.get_under_presses();
        let minimize = find(&h, "Minimize").into_iter().max_by(|a, b| a.y.total_cmp(&b.y)).unwrap();
        let close = find(&h, "Close").into_iter().max_by(|a, b| a.y.total_cmp(&b.y)).unwrap();
        // The middle of the block, between its first and last button.
        let x = (minimize.x + close.x + close.w) / 2.0;
        let y = close.y + close.h / 2.0;
        mouse_move(h.window(), x, y);
        mouse_down(h.window(), x, y);
        mouse_up(h.window(), x, y);
        assert_eq!(h.get_under_presses(), before, "{style:?}");
    }
}

#[derive(Debug, Clone, Copy)]
enum Style {
    Windows,
    Mac,
    Linux,
}

impl From<Style> for PlatformStyle {
    fn from(style: Style) -> Self {
        match style {
            Style::Windows => PlatformStyle::Windows,
            Style::Mac => PlatformStyle::Mac,
            Style::Linux => PlatformStyle::Linux,
        }
    }
}

// Windows draws three wide caption buttons, the bar's height less its hairline, flush right.
fn windows_draws_wide_caption_buttons_at_the_end() {
    let (h, _) = harness();
    h.set_style(ChromeStyle::Windows);
    advance(16);
    let close = bar_close(&h);
    assert_eq!((close.w, close.h), (46.0, 37.0));
    assert_eq!(close.x + close.w, 600.0);
    let maximize = find(&h, "Maximize").into_iter().min_by(|a, b| a.y.total_cmp(&b.y)).unwrap();
    assert_eq!(maximize.x + 46.0, close.x);
}

// At 280 px a Windows bar's long title gives way, and its close button ends inside the bar.
fn a_long_title_leaves_the_caption_buttons_in_the_bar() {
    let (h, _) = harness();
    h.set_style(ChromeStyle::Windows);
    h.set_bar_width(280.0);
    h.set_heading("A very long document title that cannot possibly fit in the bar".into());
    advance(16);
    let close = bar_close(&h);
    assert_eq!(close.x + close.w, 280.0);
    let title = one(&h, "A very long document title that cannot possibly fit in the bar");
    assert!(title.x + title.w <= close.x - 46.0 * 2.0 - 8.0 + 0.5, "title ends at {}", title.x + title.w);
}

// A Windows bar with no title still keeps its buttons at its right end.
fn an_untitled_windows_bar_keeps_its_buttons_at_the_end() {
    let (h, _) = harness();
    h.set_style(ChromeStyle::Windows);
    h.set_heading("".into());
    advance(16);
    let close = bar_close(&h);
    assert_eq!(close.x + close.w, 600.0);
}

// A short title sits at the bar's center, whatever stands at either end.
fn a_short_title_sits_at_the_center() {
    let (h, _) = harness();
    for style in [ChromeStyle::Linux, ChromeStyle::Mac] {
        h.set_style(style);
        h.set_heading("Notes".into());
        advance(16);
        let title = one(&h, "Notes");
        assert!((title.x + title.w / 2.0 - 300.0).abs() <= 1.0, "{style:?}: centered at {}", title.x + title.w / 2.0);
    }
}

// A long centered title ends before the Linux round buttons.
fn a_long_centered_title_ends_before_the_buttons() {
    let (h, _) = harness();
    h.set_bar_width(320.0);
    let text = "A long title that keeps going well past the middle of the bar";
    h.set_heading(text.into());
    advance(16);
    let title = one(&h, text);
    let minimize = find(&h, "Minimize").into_iter().min_by(|a, b| a.y.total_cmp(&b.y)).unwrap();
    assert!(title.x + title.w <= minimize.x - 8.0 + 0.5);
    let lead = one(&h, "lead");
    assert!(title.x >= lead.x + lead.w + 8.0 - 0.5);
}

// The drawn Close runs the owner's handler, or closes through the window host.
fn the_drawn_close_runs_the_owner_or_the_window() {
    let (h, counts) = harness();
    let close = bar_close(&h);
    click(h.window(), close.x + close.w / 2.0, close.y + close.h / 2.0);
    assert_eq!((h.get_closes(), counts.closes.get()), (1, 0));
    h.set_handles(false);
    advance(16);
    click(h.window(), close.x + close.w / 2.0, close.y + close.h / 2.0);
    assert_eq!((h.get_closes(), counts.closes.get()), (1, 1));
}

// The system's lights get room on macOS and no drawn controls; full screen takes the room away.
fn system_lights_get_room() {
    let (h, _) = harness();
    h.set_style(ChromeStyle::System);
    h.global::<WindowChrome>().set_system_lights(true);
    advance(16);
    assert!(find(&h, "Close").len() == 1, "only the second set's Close is drawn");
    assert_eq!(one(&h, "lead").x, 78.0);
    h.global::<WindowChrome>().set_fullscreen(true);
    advance(16);
    assert_eq!(one(&h, "lead").x, 12.0);
}

// Corners win over edges and the middle is free; a tiled edge grabs nothing.
fn resize_edges_follow_the_frame() {
    let (h, _) = harness();
    let chrome = h.global::<WindowChrome>();
    let free = Tiling { top: false, left: false, right: false, bottom: false };
    let at = |x: f32, y: f32, tiling: &Tiling| chrome.invoke_resize_edge(x, y, 10.0, 400.0, 300.0, tiling.clone());
    assert_eq!(at(2.0, 2.0, &free), ResizeEdge::TopLeft);
    assert_eq!(at(398.0, 5.0, &free), ResizeEdge::TopRight);
    assert_eq!(at(200.0, 3.0, &free), ResizeEdge::Top);
    assert_eq!(at(1.0, 299.0, &free), ResizeEdge::BottomLeft);
    assert_eq!(at(395.0, 295.0, &free), ResizeEdge::BottomRight);
    assert_eq!(at(200.0, 295.0, &free), ResizeEdge::Bottom);
    assert_eq!(at(4.0, 150.0, &free), ResizeEdge::Left);
    assert_eq!(at(396.0, 150.0, &free), ResizeEdge::Right);
    assert_eq!(at(200.0, 150.0, &free), ResizeEdge::None);
    let left = Tiling { top: true, left: true, bottom: true, right: false };
    assert_eq!(at(4.0, 150.0, &left), ResizeEdge::None);
    assert_eq!(at(200.0, 3.0, &left), ResizeEdge::None);
    assert_eq!(at(398.0, 5.0, &left), ResizeEdge::Right);
    assert_eq!(at(396.0, 295.0, &left), ResizeEdge::Right);
    let tiled = Tiling { top: true, left: true, bottom: true, right: true };
    for (x, y) in [(2.0, 2.0), (396.0, 150.0), (200.0, 295.0), (398.0, 298.0)] {
        assert_eq!(at(x, y, &tiled), ResizeEdge::None);
    }
}

// With client decorations, a press in the margin starts a resize from that edge.
fn a_press_in_the_margin_resizes() {
    let (h, _) = harness();
    let edges: Rc<std::cell::RefCell<Vec<ResizeEdge>>> = Rc::default();
    let seen = edges.clone();
    let chrome = h.global::<WindowChrome>();
    chrome.on_start_resize(move |edge| seen.borrow_mut().push(edge));
    chrome.set_client_decorated(true);
    advance(16);
    // The border box starts below the bar (38) and the controls row (50).
    let top = 88.0;
    mouse_move(h.window(), 300.0, top + 3.0);
    mouse_down(h.window(), 300.0, top + 3.0);
    mouse_up(h.window(), 300.0, top + 3.0);
    mouse_move(h.window(), 300.0, top + 100.0);
    mouse_down(h.window(), 300.0, top + 100.0);
    mouse_up(h.window(), 300.0, top + 100.0);
    assert_eq!(*edges.borrow(), vec![ResizeEdge::Top]);
}

// A routed bar takes system closes: the host bumps `close-asked` and the bar runs its handler.
fn a_routed_bar_takes_system_closes() {
    let (h, _) = harness();
    h.set_style(ChromeStyle::System);
    advance(16);
    assert!(h.global::<WindowChrome>().get_close_routed());
    let chrome = h.global::<WindowChrome>();
    chrome.set_close_asked(chrome.get_close_asked() + 1);
    advance(16);
    assert_eq!(h.get_closes(), 1);
    h.set_handles(false);
    advance(16);
    assert!(!h.global::<WindowChrome>().get_close_routed());
}

// Wired: the drawn Close asks the close check, and so does a system close; a routed bar takes
// the system's close instead.
fn the_window_host_asks_the_close_check() {
    let h = Harness::new().unwrap();
    h.global::<WindowChrome>().set_system_lights(false);
    let host = ely_slint::wire_window!(h);
    show(&h);
    advance(16);
    let asked = Rc::new(Cell::new(0));
    let seen = asked.clone();
    host.guard_close(move || {
        seen.set(seen.get() + 1);
        false
    });
    h.set_handles(false);
    advance(16);
    let close = bar_close(&h);
    click(h.window(), close.x + close.w / 2.0, close.y + close.h / 2.0);
    assert_eq!(asked.get(), 1);
    assert!(h.window().is_visible());
    h.window().dispatch_event(slint::platform::WindowEvent::CloseRequested);
    assert_eq!(asked.get(), 2);
    assert!(h.window().is_visible());
    h.set_style(ChromeStyle::System);
    h.set_handles(true);
    advance(16);
    h.window().dispatch_event(slint::platform::WindowEvent::CloseRequested);
    advance(16);
    assert_eq!((asked.get(), h.get_closes()), (2, 1));
    host.guard_close(|| true);
    h.set_handles(false);
    advance(16);
    h.window().dispatch_event(slint::platform::WindowEvent::CloseRequested);
    assert!(!h.window().is_visible());
}

cases!(title_bar: a_wobbling_press_on_a_linux_button_clicks_it, a_wobbling_press_on_the_bar_moves_the_window, a_wobbling_press_on_a_slot_moves_no_window, window_controls_occlude_what_lies_under, windows_draws_wide_caption_buttons_at_the_end, a_long_title_leaves_the_caption_buttons_in_the_bar, an_untitled_windows_bar_keeps_its_buttons_at_the_end, a_short_title_sits_at_the_center, a_long_centered_title_ends_before_the_buttons, the_drawn_close_runs_the_owner_or_the_window, system_lights_get_room, resize_edges_follow_the_frame, a_press_in_the_margin_resizes, a_routed_bar_takes_system_closes, the_window_host_asks_the_close_check);
