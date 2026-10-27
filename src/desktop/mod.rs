//! Desktop integrations (ely-desktop/src/native.rs): the tray icon and the Dock badge. Behind the
//! `desktop` feature.
//!
//! A tray hands each pick to the app on Slint's event loop (`slint::invoke_from_event_loop`), as
//! gpui hands it to the app's run loop; make the tray on the event loop's thread.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::atomic::{AtomicU32, Ordering};

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod mac;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "macos")]
type Inner = mac::StatusItem;
#[cfg(target_os = "windows")]
type Inner = windows::Tray;
#[cfg(target_os = "linux")]
type Inner = linux::Tray;
#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
type Inner = none::Tray;

/// Elsewhere: no tray.
#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
mod none {
    pub(crate) struct Tray;

    impl Tray {
        pub(crate) fn new(_: &super::TrayLook, _: Vec<super::TrayItem>, _: super::Route) -> anyhow::Result<Self> {
            anyhow::bail!("tray icons need macOS, Windows or Linux")
        }

        pub(crate) fn pick(&self, _: usize) {}
    }
}

/// One entry in a tray menu.
#[derive(Clone, Debug)]
pub enum TrayItem {
    /// A row; a pick hands `on_pick` its place among all the items.
    Action(String),
    /// A line between rows.
    Separator,
}

/// How the icon looks: macOS draws the symbol, others the PNG.
pub struct TrayLook {
    /// An SF Symbol, for the macOS menu bar.
    pub symbol: &'static str,
    /// A PNG, for the Windows tray and Linux's StatusNotifierItem.
    pub png: &'static [u8],
    /// The app's name, shown on hover.
    pub name: String,
}

type OnPick = Rc<dyn Fn(usize)>;

thread_local! {
    // Each tray's pick handler, by tray, on the event loop's thread.
    static PICKS: RefCell<HashMap<u32, OnPick>> = RefCell::new(HashMap::new());
}

/// Hands a pick to the app, from any thread.
#[derive(Clone, Copy)]
struct Route {
    tray: u32,
}

impl Route {
    #[cfg_attr(
        not(any(target_os = "linux", target_os = "macos", target_os = "windows")),
        expect(dead_code, reason = "no tray takes it")
    )]
    fn pick(&self, index: usize) {
        log::info!("tray: picked item {index}");
        let tray = self.tray;
        let handed = slint::invoke_from_event_loop(move || {
            let on_pick = PICKS.with(|picks| picks.borrow().get(&tray).cloned());
            if let Some(on_pick) = on_pick {
                on_pick(index);
            }
        });
        if let Err(error) = handed {
            log::error!("tray: no event loop for the pick: {error}");
        }
    }
}

/// An icon in the menu bar or tray with a menu. Dropping it removes it.
///
/// Platforms: macOS, Windows, Linux.
pub struct TrayIcon {
    inner: Inner,
    tray: u32,
}

impl TrayIcon {
    /// `on_pick` gets the chosen item's index on the event loop; a left click picks the first.
    pub fn new(look: TrayLook, items: Vec<TrayItem>, on_pick: impl Fn(usize) + 'static) -> anyhow::Result<Self> {
        static NEXT: AtomicU32 = AtomicU32::new(0);
        let tray = NEXT.fetch_add(1, Ordering::Relaxed);
        PICKS.with(|picks| picks.borrow_mut().insert(tray, Rc::new(on_pick)));
        match open(&look, items, Route { tray }) {
            Ok(inner) => Ok(Self { inner, tray }),
            Err(error) => {
                PICKS.with(|picks| picks.borrow_mut().remove(&tray));
                Err(error)
            }
        }
    }

    /// Chooses item `index` as a click would.
    pub fn pick(&self, index: usize) {
        self.inner.pick(index);
    }
}

impl Drop for TrayIcon {
    fn drop(&mut self) {
        PICKS.with(|picks| picks.borrow_mut().remove(&self.tray));
    }
}

/// The OS's tray: macOS and Windows take labels and a pick callback.
#[cfg(any(target_os = "macos", target_os = "windows"))]
fn open(look: &TrayLook, items: Vec<TrayItem>, route: Route) -> anyhow::Result<Inner> {
    let entries: Vec<Option<&str>> = (items.iter())
        .map(|item| match item {
            TrayItem::Action(label) => Some(label.as_str()),
            TrayItem::Separator => None,
        })
        .collect();
    let on_pick = Box::new(move |index| route.pick(index));
    #[cfg(target_os = "macos")]
    return Inner::new(look.symbol, &look.name, &entries, on_pick);
    #[cfg(target_os = "windows")]
    return Inner::new(look.png, &look.name, &entries, on_pick);
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn open(look: &TrayLook, items: Vec<TrayItem>, route: Route) -> anyhow::Result<Inner> {
    Inner::new(look, items, route)
}

/// Sets or clears the badge on the app's Dock icon.
///
/// Platforms: macOS.
pub fn set_dock_badge(label: Option<&str>) -> anyhow::Result<()> {
    #[cfg(target_os = "macos")]
    {
        mac::set_dock_badge(label);
        log::info!("dock badge: {label:?}");
        Ok(())
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = label;
        anyhow::bail!("dock badges need macOS in this build")
    }
}

/// The badge the Dock shows now.
///
/// Platforms: macOS.
pub fn dock_badge() -> anyhow::Result<Option<String>> {
    #[cfg(target_os = "macos")]
    return Ok(mac::dock_badge());
    #[cfg(not(target_os = "macos"))]
    anyhow::bail!("dock badges need macOS in this build")
}

/// Points the process at a private session bus.
#[cfg(target_os = "linux")]
#[doc(hidden)]
pub fn use_session_bus(address: &str) {
    // SAFETY: only the one test that owns the bus reads or writes this variable.
    unsafe { std::env::set_var("DBUS_SESSION_BUS_ADDRESS", address) };
}
