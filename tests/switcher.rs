// WindowManager, WindowSwitcher and MiniWindow, after ely-core/src/shell/windows.rs and
// switcher.rs.
#[macro_use]
mod common;
use common::*;
use ely_slint::windows::WindowManager;
use i_slint_backend_testing::ElementHandle;
use slint::ComponentHandle;
use slint::platform::Key;

slint::slint! {
    import { WindowSwitcher, ManagedWindows, MiniWindow, MiniWindowActs, WindowChrome, ResizeEdge } from "ui/index.slint";
    export { ManagedWindows, MiniWindowActs, WindowChrome, ResizeEdge }
    export component Harness inherits Window {
        width: 600px;
        height: 400px;
        in-out property <bool> switching: true;
        out property <int> closes;
        if root.switching: WindowSwitcher {
            close => {
                root.closes += 1;
                root.switching = false;
            }
        }
    }

    export component Doc inherits Window {
        width: 300px;
        height: 200px;
    }

    export component Timer inherits MiniWindow {
        heading: "Timer";
        out property <bool> bar: root.bar-shown;
        Text { text: "12:00"; }
    }
}

struct Box_ {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

fn find(h: &impl i_slint_backend_testing::ElementRoot, label: &str) -> Vec<ElementHandle> {
    ElementHandle::find_by_accessible_label(h, label).collect()
}

fn bounds(e: &ElementHandle) -> Box_ {
    let (at, size) = (e.absolute_position(), e.size());
    Box_ {
        x: at.x,
        y: at.y,
        w: size.width,
        h: size.height,
    }
}

fn selected(h: &Harness, titles: &[&str]) -> Option<String> {
    titles
        .iter()
        .find(|t| {
            find(h, t)
                .iter()
                .any(|e| e.accessible_item_selected() == Some(true))
        })
        .map(|t| t.to_string())
}

fn docs() -> Vec<Doc> {
    let docs: Vec<Doc> = (0..3).map(|_| Doc::new().unwrap()).collect();
    for (doc, title) in docs.iter().zip(["Inbox", "Roadmap", "Notes"]) {
        WindowManager::open(doc, title).unwrap();
    }
    docs
}

fn open_switcher() -> Harness {
    let h = Harness::new().unwrap();
    ely_slint::wire_managed_windows!(h);
    show(&h);
    advance(16);
    h
}

// Windows open oldest first, each a title bar down and right of the newest; closed ones drop out.
fn the_manager_keeps_open_windows_oldest_first() {
    let docs = docs();
    let titles: Vec<String> = WindowManager::windows()
        .into_iter()
        .map(|(_, t)| t.to_string())
        .collect();
    assert_eq!(titles, ["Inbox", "Roadmap", "Notes"]);
    docs[1].hide().unwrap();
    let titles: Vec<String> = WindowManager::windows()
        .into_iter()
        .map(|(_, t)| t.to_string())
        .collect();
    assert_eq!(titles, ["Inbox", "Notes"]);
    for doc in &docs {
        doc.hide().unwrap();
    }
    assert!(WindowManager::windows().is_empty());
}

// Down and Tab step on, Up and Shift+Tab back, wrapping; Enter switches on release and closes.
fn the_switcher_steps_and_switches_on_enter() {
    let docs = docs();
    let h = open_switcher();
    let titles = ["Inbox", "Roadmap", "Notes"];
    let panel = bounds(&find(&h, "Windows")[0]);
    assert_eq!(panel.w, 380.0);
    assert_eq!(bounds(&find(&h, "Inbox")[0]).h, 32.0);
    assert_eq!(selected(&h, &titles).as_deref(), Some("Inbox"));
    tap(h.window(), Key::DownArrow);
    assert_eq!(selected(&h, &titles).as_deref(), Some("Roadmap"));
    tab(h.window());
    assert_eq!(selected(&h, &titles).as_deref(), Some("Notes"));
    tab(h.window());
    assert_eq!(selected(&h, &titles).as_deref(), Some("Inbox"), "wraps");
    shift_tab(h.window());
    assert_eq!(selected(&h, &titles).as_deref(), Some("Notes"));
    tap(h.window(), Key::UpArrow);
    assert_eq!(selected(&h, &titles).as_deref(), Some("Roadmap"));
    key_down(h.window(), Key::Return);
    assert_eq!(h.get_closes(), 0, "a press does not switch yet");
    key_up(h.window(), Key::Return);
    assert_eq!(h.get_closes(), 1);
    for doc in &docs {
        doc.hide().unwrap();
    }
}

// Escape or a press on the scrim closes; a click on a row switches.
fn the_switcher_closes_on_escape_and_the_scrim() {
    let docs = docs();
    let h = open_switcher();
    tap(h.window(), Key::Escape);
    assert_eq!(h.get_closes(), 1);
    h.set_switching(true);
    advance(16);
    click(h.window(), 10.0, 10.0);
    assert_eq!(h.get_closes(), 2);
    h.set_switching(true);
    advance(16);
    let row = bounds(&find(&h, "Notes")[0]);
    click(h.window(), row.x + row.w / 2.0, row.y + row.h / 2.0);
    assert_eq!(h.get_closes(), 3);
    for doc in &docs {
        doc.hide().unwrap();
    }
}

// With none open it says so.
fn an_empty_switcher_says_so() {
    let h = open_switcher();
    assert!(!find(&h, "No managed windows are open.").is_empty());
    tap(h.window(), Key::Return);
    assert_eq!(
        h.get_closes(),
        0,
        "Enter with nothing to switch to keeps it"
    );
}

// The mini window's bar shows on hover; expand hides it and returns to the opener; without an
// opener there is no expand; close hides it.
fn the_mini_window_bar_shows_on_hover() {
    let opener = Doc::new().unwrap();
    show(&opener);
    let mini = Timer::new().unwrap();
    ely_slint::wire_mini_window!(mini, Some(opener.as_weak()));
    ely_slint::window::open_mini(mini.window(), "Timer", None).unwrap();
    advance(16);
    let size = mini
        .window()
        .size()
        .to_logical(mini.window().scale_factor());
    assert_eq!((size.width, size.height), (240.0, 120.0));
    assert!(!mini.get_bar());
    mouse_move(mini.window(), 120.0, 80.0);
    advance(16);
    assert!(mini.get_bar());
    assert!(!find(&mini, "maximize-2").is_empty() && !find(&mini, "x").is_empty());
    let expand = bounds(&find(&mini, "maximize-2")[0]);
    assert_eq!(expand.y + expand.h / 2.0, 31.0 / 2.0);
    click(
        mini.window(),
        expand.x + expand.w / 2.0,
        expand.y + expand.h / 2.0,
    );
    assert!(!mini.window().is_visible());

    let alone = Timer::new().unwrap();
    ely_slint::wire_mini_window!(alone, None::<slint::Weak<Doc>>);
    ely_slint::window::open_mini(alone.window(), "Timer", None).unwrap();
    mouse_move(alone.window(), 120.0, 80.0);
    advance(16);
    assert!(find(&alone, "maximize-2").is_empty());
    let x = bounds(&find(&alone, "x")[0]);
    click(alone.window(), x.x + x.w / 2.0, x.y + x.h / 2.0);
    assert!(!alone.window().is_visible());
    opener.hide().unwrap();
}

cases!(switcher: the_manager_keeps_open_windows_oldest_first, the_switcher_steps_and_switches_on_enter, the_switcher_closes_on_escape_and_the_scrim, an_empty_switcher_says_so, the_mini_window_bar_shows_on_hover);
