//! Focus helpers for `FocusTrap` (ely-core `FocusScope::trap`).

use slint::platform::{Key, WindowEvent};

/// Moves keyboard focus one Tab stop, forward or back, as a Tab or Shift-Tab press would.
///
/// Wire it to the `FocusSteps.step` callback once per window so `FocusTrap` can wrap:
///
/// ```ignore
/// let weak = app.as_weak();
/// app.global::<FocusSteps>().on_step(move |forward| {
///     if let Some(app) = weak.upgrade() {
///         ely_slint::focus::step(app.window(), forward);
///     }
/// });
/// ```
pub fn step(window: &slint::Window, forward: bool) {
    let key = if forward { Key::Tab } else { Key::Backtab };
    window.dispatch_event(WindowEvent::KeyPressed { text: key.into() });
    window.dispatch_event(WindowEvent::KeyReleased { text: key.into() });
}
