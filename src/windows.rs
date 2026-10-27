//! The windows an app opens through `WindowManager` (ely-core/src/shell/windows.rs): oldest
//! first, with their titles; each opens one title bar down and right of the newest, the first
//! centered. `WindowSwitcher` lists them through the `ManagedWindows` global, which
//! `wire_managed_windows!` connects here.

use std::cell::RefCell;

use slint::winit_030::WinitWindowAccessor;
use slint::{ComponentHandle, LogicalPosition, SharedString};

// The title bar's height: how far each window opens from the newest.
const STEP: f32 = 38.0;

struct Managed {
    id: usize,
    title: SharedString,
    origin: LogicalPosition,
    open: Box<dyn Fn() -> bool>,
    focus: Box<dyn Fn()>,
}

thread_local! {
    static MANAGED: RefCell<(usize, Vec<Managed>)> = const { RefCell::new((0, Vec::new())) };
}

// Where a window opens after the newest one.
fn cascade(newest: LogicalPosition) -> LogicalPosition {
    LogicalPosition::new(newest.x + STEP, newest.y + STEP)
}

/// The managed windows of this thread's event loop.
pub struct WindowManager;

impl WindowManager {
    /// Shows `component`'s window one title bar below and right of the newest managed window,
    /// or centered when it is the first, titled `title`. Returns its id.
    pub fn open<C: ComponentHandle + 'static>(
        component: &C,
        title: &str,
    ) -> Result<usize, slint::PlatformError> {
        Self::windows();
        let window = component.window();
        let newest = MANAGED.with(|m| m.borrow().1.last().map(|w| w.origin));
        match newest {
            Some(at) => {
                window.set_position(cascade(at));
                window.show()?;
            }
            None => {
                window.show()?;
                crate::window::center(window);
            }
        }
        window.with_winit_window(|w| w.set_title(title));
        let origin = window.position().to_logical(window.scale_factor());
        let weak = component.as_weak();
        let open = {
            let weak = weak.clone();
            Box::new(move || weak.upgrade().is_some_and(|c| c.window().is_visible()))
        };
        let focus = Box::new(move || {
            if let Some(c) = weak.upgrade() {
                c.window().with_winit_window(|w| w.focus_window());
            }
        });
        let id = MANAGED.with(|m| {
            let mut m = m.borrow_mut();
            m.0 += 1;
            let id = m.0;
            m.1.push(Managed {
                id,
                title: title.into(),
                origin,
                open,
                focus,
            });
            id
        });
        log::info!("window manager: opened {title}");
        Ok(id)
    }

    /// Managed windows still open, oldest first: their ids and titles.
    pub fn windows() -> Vec<(usize, SharedString)> {
        MANAGED.with(|m| {
            let mut m = m.borrow_mut();
            m.1.retain(|w| (w.open)());
            m.1.iter().map(|w| (w.id, w.title.clone())).collect()
        })
    }

    /// Brings the window with `id` to the front.
    pub fn focus(id: usize) -> Result<(), String> {
        log::info!("window manager: focus {id}");
        MANAGED.with(|m| {
            let m = m.borrow();
            let window =
                m.1.iter()
                    .find(|w| w.id == id)
                    .ok_or(format!("no managed window {id}"))?;
            (window.focus)();
            Ok(())
        })
    }
}

/// Connects a window's `ManagedWindows` global to the `WindowManager`. `ManagedWindows` must be
/// in scope (export it from your `.slint`).
#[macro_export]
macro_rules! wire_managed_windows {
    ($app:expr) => {{
        use ::std::cell::RefCell;
        use ::std::rc::Rc;
        use $crate::windows::WindowManager;

        let ids: Rc<RefCell<Vec<usize>>> = Rc::default();
        let weak = $app.as_weak();
        let listed = ids.clone();
        $app.global::<ManagedWindows>().on_list(move || {
            let Some(app) = weak.upgrade() else { return };
            let windows = WindowManager::windows();
            *listed.borrow_mut() = windows.iter().map(|(id, _)| *id).collect();
            let titles: Vec<::slint::SharedString> =
                windows.into_iter().map(|(_, title)| title).collect();
            app.global::<ManagedWindows>()
                .set_titles(::slint::ModelRc::from(Rc::new(::slint::VecModel::from(
                    titles,
                ))));
        });
        $app.global::<ManagedWindows>().on_focus(move |index| {
            let id = ids.borrow().get(index as usize).copied();
            match id.map(WindowManager::focus) {
                Some(Ok(())) => {}
                Some(Err(error)) => $crate::log::error!("window switcher: {error}"),
                None => $crate::log::error!("window switcher: no window at {index}"),
            }
        });
        // A switcher made with the window asked before this wiring.
        $app.global::<ManagedWindows>().invoke_list();
    }};
}

#[cfg(test)]
mod tests {
    use super::cascade;
    use slint::LogicalPosition;

    #[test]
    fn each_window_opens_a_title_bar_down_and_right() {
        assert_eq!(
            cascade(LogicalPosition::new(100.0, 80.0)),
            LogicalPosition::new(138.0, 118.0)
        );
    }
}
