//! The window acts behind the `WindowChrome` global (ely-core/src/shell/chrome.rs, frame.rs,
//! closing.rs and status.rs): moving, resizing, minimizing, zooming and closing a window, its
//! close check, and its zoom. `wire_window!` connects a window's `WindowChrome` to these.
//!
//! Moves and resizes go through winit (`drag_window`, `drag_resize_window`); a window that winit
//! does not back (the testing backend) logs them and stays put.

use std::cell::{Cell, RefCell};
use std::rc::Rc;

use slint::winit_030::{EventResult, WinitWindowAccessor, winit};

/// A border or corner a resize starts from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edge {
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    TopLeft,
}

impl Edge {
    fn direction(self) -> winit::window::ResizeDirection {
        use winit::window::ResizeDirection as Direction;
        match self {
            Edge::Top => Direction::North,
            Edge::TopRight => Direction::NorthEast,
            Edge::Right => Direction::East,
            Edge::BottomRight => Direction::SouthEast,
            Edge::Bottom => Direction::South,
            Edge::BottomLeft => Direction::SouthWest,
            Edge::Left => Direction::West,
            Edge::TopLeft => Direction::NorthWest,
        }
    }
}

/// What a window is doing, for `WindowChrome`'s properties.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WindowState {
    pub active: bool,
    pub maximized: bool,
    pub fullscreen: bool,
    pub client_decorated: bool,
    pub offers_minimize: bool,
    pub offers_maximize: bool,
}

/// Zoom steps, in percent (status.rs `STEPS`); ZoomControl walks them.
pub const ZOOM_STEPS: [i32; 11] = [50, 67, 75, 80, 90, 100, 110, 125, 150, 175, 200];

type Check = Rc<dyn Fn() -> bool>;

/// One window's close check and zoom.
#[derive(Default)]
pub struct WindowHost {
    check: RefCell<Option<Check>>,
    // The scale factor at 100 %, kept from the first zoom.
    base_scale: Cell<Option<f32>>,
    active: Cell<Option<bool>>,
}

impl WindowHost {
    pub fn new() -> Self {
        Self::default()
    }

    /// Asks `check` before any close removes the window, system or drawn (`guard_close`); false
    /// keeps it.
    pub fn guard_close(&self, check: impl Fn() -> bool + 'static) {
        *self.check.borrow_mut() = Some(Rc::new(check));
    }

    /// Whether the close check, if any, lets the window close. Logged when it keeps it.
    pub fn may_close(&self) -> bool {
        let check = self.check.borrow().clone();
        let allowed = check.is_none_or(|check| check());
        if !allowed {
            log::info!("window controls: the close check kept the window");
        }
        allowed
    }

    /// Closes `window` once its check lets it.
    pub fn close(&self, window: &slint::Window) {
        log::info!("window controls: Close");
        if !self.may_close() {
            return;
        }
        if let Err(error) = window.hide() {
            log::error!("window: hiding failed: {error}");
        }
    }

    /// Shows `window` at `percent` of its natural size, by scaling it (gap G40).
    pub fn set_zoom(&self, window: &slint::Window, percent: i32) -> i32 {
        let base = self.base_scale.get().unwrap_or_else(|| window.scale_factor());
        self.base_scale.set(Some(base));
        let percent = percent.clamp(ZOOM_STEPS[0], ZOOM_STEPS[ZOOM_STEPS.len() - 1]);
        log::info!("zoom: {percent}%");
        window.dispatch_event(slint::platform::WindowEvent::ScaleFactorChanged {
            scale_factor: base * percent as f32 / 100.0,
        });
        window.request_redraw();
        percent
    }

    /// What `window` is doing now.
    pub fn state(&self, window: &slint::Window) -> WindowState {
        let (client_decorated, offers_minimize, offers_maximize) = window
            .with_winit_window(|w| {
                let buttons = w.enabled_buttons();
                (
                    !w.is_decorated(),
                    buttons.contains(winit::window::WindowButtons::MINIMIZE),
                    buttons.contains(winit::window::WindowButtons::MAXIMIZE),
                )
            })
            .unwrap_or((false, true, true));
        WindowState {
            active: self.active.get().unwrap_or(true),
            maximized: window.is_maximized(),
            fullscreen: window.is_fullscreen(),
            client_decorated,
            offers_minimize,
            offers_maximize,
        }
    }

    /// Calls `changed` with the window's state now, and again after each winit event that can
    /// change it.
    pub fn follow(self: &Rc<Self>, window: &slint::Window, changed: impl Fn(WindowState) + 'static) {
        changed(self.state(window));
        let host = Rc::downgrade(self);
        window.on_winit_window_event(move |window, event| {
            use winit::event::WindowEvent as Event;
            let Some(host) = host.upgrade() else {
                return EventResult::Propagate;
            };
            if let Event::Focused(active) = event {
                host.active.set(Some(*active));
            }
            if matches!(event, Event::Focused(_) | Event::Resized(_) | Event::Moved(_) | Event::Occluded(_)) {
                changed(host.state(window));
            }
            EventResult::Propagate
        });
    }
}

/// Starts moving `window` with the pointer.
pub fn start_move(window: &slint::Window) {
    let moved = window.with_winit_window(|w| w.drag_window());
    match moved {
        Some(Ok(())) => {}
        Some(Err(error)) => log::error!("window: move failed: {error}"),
        None => log::info!("window: move asked of a window winit does not back"),
    }
}

/// Starts resizing `window` from `edge`.
pub fn start_resize(window: &slint::Window, edge: Edge) {
    log::info!("window: resize from {edge:?}");
    let resized = window.with_winit_window(|w| w.drag_resize_window(edge.direction()));
    match resized {
        Some(Ok(())) => {}
        Some(Err(error)) => log::error!("window: resize failed: {error}"),
        None => log::info!("window: resize asked of a window winit does not back"),
    }
}

pub fn minimize(window: &slint::Window) {
    log::info!("window controls: Minimize");
    window.set_minimized(true);
}

/// Maximizes `window`, or restores it.
pub fn zoom(window: &slint::Window) {
    log::info!("window controls: Maximize");
    if resizable(window) {
        window.set_maximized(!window.is_maximized());
    }
}

/// A title bar's double click: zoom, as Linux does and macOS's default setting (gap G38).
pub fn title_double_click(window: &slint::Window) {
    if resizable(window) {
        window.set_maximized(!window.is_maximized());
    }
}

// A window that does not resize does not zoom either, as the systems keep it.
fn resizable(window: &slint::Window) -> bool {
    window.with_winit_window(|w| w.is_resizable()).unwrap_or(true)
}

/// Shows a `HostedWindow` as gpui's `open_hosted` does: centered on its display, not resizable,
/// not minimizable. Wire its `WindowChrome` with `wire_window!` first.
pub fn open_hosted(window: &slint::Window, title: &str) -> Result<(), slint::PlatformError> {
    window.show()?;
    settle(window, winit::window::WindowButtons::CLOSE);
    log::info!("window: opened {title}");
    Ok(())
}

/// Shows a frameless `HostedWindow` as gpui's `SplashScreen::open` does: centered, neither
/// resizable nor minimizable. Hide it once the app has loaded.
pub fn open_splash(window: &slint::Window) -> Result<(), slint::PlatformError> {
    window.show()?;
    settle(window, winit::window::WindowButtons::empty());
    log::info!("splash: shown");
    Ok(())
}

// Fixes the window's size and buttons and centers it on its display. The window shows for a
// moment where the system placed it first (G44).
fn settle(window: &slint::Window, buttons: winit::window::WindowButtons) {
    window.with_winit_window(|w| {
        w.set_resizable(false);
        w.set_enabled_buttons(buttons);
        if let Some(display) = w.current_monitor() {
            let (at, room, size) = (display.position(), display.size(), w.outer_size());
            let x = at.x + (room.width as i32 - size.width as i32) / 2;
            let y = at.y + (room.height as i32 - size.height as i32) / 2;
            w.set_outer_position(winit::dpi::PhysicalPosition::new(x, y));
        }
    });
}

/// Connects a window component's `WindowChrome` to its window. Returns the `Rc<WindowHost>`,
/// whose `guard_close` sets the close check. `ResizeEdge` must be in scope (export it from
/// your `.slint`).
#[macro_export]
macro_rules! wire_window {
    ($app:expr) => {{
        use ::std::rc::Rc;
        use $crate::window::{self, Edge, WindowHost};

        let host = Rc::new(WindowHost::new());
        let chrome = $app.global::<WindowChrome>();
        let weak = $app.as_weak();
        let act = |run: fn(&::slint::Window)| {
            let weak = weak.clone();
            move || {
                if let Some(app) = weak.upgrade() {
                    run(app.window());
                }
            }
        };
        chrome.on_start_move(act(window::start_move));
        chrome.on_minimize(act(window::minimize));
        chrome.on_zoom(act(window::zoom));
        chrome.on_title_double_click(act(window::title_double_click));
        chrome.on_start_resize({
            let weak = weak.clone();
            move |edge| {
                let edge = match edge {
                    ResizeEdge::Top => Edge::Top,
                    ResizeEdge::TopRight => Edge::TopRight,
                    ResizeEdge::Right => Edge::Right,
                    ResizeEdge::BottomRight => Edge::BottomRight,
                    ResizeEdge::Bottom => Edge::Bottom,
                    ResizeEdge::BottomLeft => Edge::BottomLeft,
                    ResizeEdge::Left => Edge::Left,
                    ResizeEdge::TopLeft => Edge::TopLeft,
                    _ => return,
                };
                if let Some(app) = weak.upgrade() {
                    window::start_resize(app.window(), edge);
                }
            }
        });
        chrome.on_close({
            let (weak, host) = (weak.clone(), host.clone());
            move || {
                if let Some(app) = weak.upgrade() {
                    host.close(app.window());
                }
            }
        });
        chrome.on_set_zoom({
            let (weak, host) = (weak.clone(), host.clone());
            move |percent| {
                if let Some(app) = weak.upgrade() {
                    let shown = host.set_zoom(app.window(), percent);
                    app.global::<WindowChrome>().set_zoom_percent(shown);
                }
            }
        });
        $app.window().on_close_requested({
            let (weak, host) = (weak.clone(), host.clone());
            move || {
                let Some(app) = weak.upgrade() else {
                    return ::slint::CloseRequestResponse::HideWindow;
                };
                let chrome = app.global::<WindowChrome>();
                if chrome.get_close_routed() {
                    $crate::log::info!("title bar: system close routed to close-requested");
                    chrome.set_close_asked(chrome.get_close_asked() + 1);
                    return ::slint::CloseRequestResponse::KeepWindowShown;
                }
                if host.may_close() {
                    ::slint::CloseRequestResponse::HideWindow
                } else {
                    ::slint::CloseRequestResponse::KeepWindowShown
                }
            }
        });
        host.follow($app.window(), {
            let weak = weak.clone();
            move |state| {
                let Some(app) = weak.upgrade() else { return };
                let chrome = app.global::<WindowChrome>();
                let tiled = state.maximized || state.fullscreen;
                chrome.set_active(state.active);
                chrome.set_maximized(state.maximized);
                chrome.set_fullscreen(state.fullscreen);
                chrome.set_client_decorated(state.client_decorated);
                chrome.set_offers_minimize(state.offers_minimize);
                chrome.set_offers_maximize(state.offers_maximize);
                chrome.set_tiling(Tiling { top: tiled, left: tiled, right: tiled, bottom: tiled });
            }
        });
        host
    }};
}
