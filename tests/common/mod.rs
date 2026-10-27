#![allow(dead_code)]

use slint::platform::{Key, WindowEvent};
use slint::{ComponentHandle, LogicalPosition, SharedString};

thread_local! {
    static INIT: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

// The testing backend is installed once per process; Slint objects must stay on that thread,
// so tests run with RUST_TEST_THREADS=1 (see .cargo/config.toml).
pub fn init() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(i_slint_backend_testing::init_integration_test_with_mock_time);
    INIT.with(|i| i.set(true));
}

pub fn key_down(w: &slint::Window, text: impl Into<SharedString>) {
    w.dispatch_event(WindowEvent::KeyPressed { text: text.into() });
}

pub fn key_up(w: &slint::Window, text: impl Into<SharedString>) {
    w.dispatch_event(WindowEvent::KeyReleased { text: text.into() });
}

pub fn tap(w: &slint::Window, text: impl Into<SharedString> + Clone) {
    key_down(w, text.clone());
    key_up(w, text);
}

pub fn tab(w: &slint::Window) {
    tap(w, Key::Tab);
}

pub fn shift_tab(w: &slint::Window) {
    key_down(w, Key::Shift);
    tap(w, Key::Tab);
    key_up(w, Key::Shift);
}

pub fn mouse_move(w: &slint::Window, x: f32, y: f32) {
    w.dispatch_event(WindowEvent::PointerMoved { position: LogicalPosition::new(x, y) });
}

pub fn mouse_down(w: &slint::Window, x: f32, y: f32) {
    w.dispatch_event(WindowEvent::PointerPressed {
        position: LogicalPosition::new(x, y),
        button: slint::platform::PointerEventButton::Left,
    });
}

pub fn mouse_up(w: &slint::Window, x: f32, y: f32) {
    w.dispatch_event(WindowEvent::PointerReleased {
        position: LogicalPosition::new(x, y),
        button: slint::platform::PointerEventButton::Left,
    });
}

pub fn click(w: &slint::Window, x: f32, y: f32) {
    mouse_move(w, x, y);
    mouse_down(w, x, y);
    mouse_up(w, x, y);
}

pub fn advance(ms: u64) {
    i_slint_backend_testing::mock_elapsed_time(std::time::Duration::from_millis(ms));
    slint::platform::update_timers_and_animations();
}

pub fn show<C: ComponentHandle>(c: &C) {
    c.show().unwrap();
}

// Runs every case on the one thread that owns the testing backend.
#[macro_export]
macro_rules! cases {
    ($suite:ident: $($name:ident),* $(,)?) => {
        #[test]
        fn $suite() {
            $crate::common::init();
            let mut failed = Vec::new();
            $(
                if std::panic::catch_unwind(|| $name()).is_err() {
                    failed.push(stringify!($name));
                } else {
                    println!("ok {}", stringify!($name));
                }
            )*
            assert!(failed.is_empty(), "failed cases: {failed:?}");
        }
    };
}

// One wheel notch at (x, y); negative `dy` scrolls down, as Slint's deltas run.
pub fn wheel(w: &slint::Window, x: f32, y: f32, dx: f32, dy: f32) {
    w.dispatch_event(WindowEvent::PointerScrolled { position: LogicalPosition::new(x, y), delta_x: dx, delta_y: dy });
}
