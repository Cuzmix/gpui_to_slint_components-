//! What the gallery binary and the capture example both hand the gallery: the Rust globals, the
//! layout chapter's dock, pane group and workspace (ely-gallery layout/systems.rs), and the Window
//! & Shell page's tabs, windows, tray and Dock badge (shell/bars.rs, windows.rs, system.rs).

use std::{any::Any, cell::RefCell, rc::Rc};

use ely_slint::{
    dock::{self, Dock, DockPanel},
    panes::{Axis, PaneGroup},
    workspace::{Workspace, WorkspaceHost},
};
use slint::ComponentHandle;

use super::*;

/// Keeps the wiring alive as long as the gallery runs.
pub struct Wired {
    _clock: slint::Timer,
    _host: Rc<WorkspaceHost>,
    _shell: Shell,
}

fn demo_dock() -> Dock {
    let panel = |id: &str, title: &str, icon: &str| DockPanel { id: id.into(), title: title.into(), icon: icon.into() };
    let mut dock = Dock::new();
    dock.add(panel("files", "Files", "folder"), dock::DockSide::Left);
    dock.add(panel("outline", "Outline", "layers"), dock::DockSide::Left);
    dock.add(panel("search", "Search", "search"), dock::DockSide::Right);
    dock.add(panel("terminal", "Terminal", "terminal"), dock::DockSide::Bottom);
    dock
}

fn demo_panes() -> PaneGroup {
    let mut group = PaneGroup::new();
    group.split(1, Axis::Horizontal);
    group
}

pub fn wire(gallery: &Gallery) -> Wired {
    ely_slint::wire_text_format!(gallery);
    ely_slint::wire_links!(gallery);
    ely_slint::wire_layout!(gallery);
    ely_slint::wire_toggles!(gallery);
    #[cfg(feature = "desktop")]
    ely_slint::wire_sharing!(gallery);
    let clock = ely_slint::wire_clock!(gallery);
    let host = ely_slint::wire_workspace!(gallery);
    host.update(|systems| {
        systems.docks.insert("demo-dock".into(), demo_dock());
        systems.groups.insert("demo-panes".into(), demo_panes());
    });

    let saved = Rc::new(RefCell::new(None::<String>));
    {
        let (host, saved, weak) = (host.clone(), saved.clone(), gallery.as_weak());
        gallery.on_workspace_save(move || {
            let Some(json) = host.read(|systems| systems.capture("demo-dock", "demo-panes")).map(|layout| layout.to_json()) else {
                return;
            };
            if let Some(gallery) = weak.upgrade() {
                let excerpt: Vec<&str> = json.lines().take(12).collect();
                gallery.set_workspace_json(excerpt.join("\n").into());
            }
            *saved.borrow_mut() = Some(json);
        });
    }
    {
        let (host, weak) = (host.clone(), gallery.as_weak());
        gallery.on_workspace_restore(move || {
            let json = saved.borrow().clone().expect("restore is disabled until a layout is saved");
            let outcome = Workspace::from_json(&json)
                .and_then(|layout| host.update(|systems| systems.apply(&layout, "demo-dock", "demo-panes")));
            let note = match outcome {
                Ok(()) => "Restored.".to_string(),
                Err(error) => format!("Refused: {error:#}"),
            };
            if let Some(gallery) = weak.upgrade() {
                gallery.set_workspace_note(note.into());
            }
        });
    }
    Wired { _clock: clock, _host: host, _shell: wire_shell(gallery) }
}

/// The Window & Shell page's state: the windows it opened, the window list's refresh, the tray.
struct Shell {
    _opened: Rc<RefCell<Vec<Box<dyn Any>>>>,
    _refresh: slint::Timer,
    _tray: Rc<RefCell<Option<Tray>>>,
}

#[cfg(feature = "desktop")]
type Tray = ely_slint::desktop::TrayIcon;
#[cfg(not(feature = "desktop"))]
type Tray = ();

#[cfg(feature = "desktop")]
const TRAY_ITEMS: [&str; 3] = ["Open gallery", "Toggle theme", "Quit"];

fn dock_reading() -> String {
    #[cfg(feature = "desktop")]
    let reading = match ely_slint::desktop::dock_badge() {
        Ok(Some(label)) => format!("The Dock shows “{label}”."),
        Ok(None) => "The Dock shows no badge.".to_string(),
        Err(error) => format!("{error:#}"),
    };
    #[cfg(not(feature = "desktop"))]
    let reading = "Dock badges need the desktop feature.".to_string();
    reading
}

fn wire_shell(gallery: &Gallery) -> Shell {
    let _ = ely_slint::wire_window!(gallery);
    ely_slint::wire_column_shell!(gallery);
    ely_slint::wire_managed_windows!(gallery);
    gallery.on_logged(|what| log::info!("gallery: {what}"));
    gallery.set_dock_reading(dock_reading().into());

    // Window tabs (bars.rs `tab_bar`), edited in place so only a new tab rises in.
    let icons = gallery.global::<Icons>();
    let tab = |id: &str, title: &str, icon: slint::Image| WindowTab { id: id.into(), title: title.into(), icon };
    let tabs = Rc::new(slint::VecModel::from(vec![
        tab("inbox", "Inbox", icons.get_inbox()),
        tab("roadmap", "Roadmap", icons.get_map()),
        tab("notes", "Meeting notes", icons.get_notebook_pen()),
    ]));
    gallery.set_tabs(slint::ModelRc::from(tabs.clone()));
    {
        let (tabs, weak) = (tabs.clone(), gallery.as_weak());
        gallery.on_tab_close(move |id| {
            use slint::Model;
            if let Some(ix) = tabs.iter().position(|tab| tab.id == id) {
                tabs.remove(ix);
            }
            if let Some(gallery) = weak.upgrade() {
                if gallery.get_selected_tab() == id {
                    gallery.set_selected_tab(tabs.row_data(0).map(|tab| tab.id).unwrap_or_default());
                }
            }
        });
    }
    {
        let (tabs, weak, next) = (tabs.clone(), gallery.as_weak(), Rc::new(RefCell::new(1)));
        gallery.on_tab_add(move || {
            let Some(gallery) = weak.upgrade() else { return };
            let number = *next.borrow();
            *next.borrow_mut() += 1;
            let id = format!("draft-{number}");
            let icon = gallery.global::<Icons>().get_file_text();
            tabs.push(tab(&id, &format!("Draft {number}"), icon));
            gallery.set_selected_tab(id.into());
        });
    }
    gallery.on_tab_reorder(move |from, to| {
        use slint::Model;
        let (from, to) = (from as usize, to as usize);
        if let Some(moved) = tabs.row_data(from) {
            tabs.remove(from);
            tabs.insert(to.min(tabs.row_count()), moved);
        }
    });

    // The windows it opens (windows.rs `trigger`, status.rs).
    let opened: Rc<RefCell<Vec<Box<dyn Any>>>> = Rc::default();
    {
        let (opened, weak) = (opened.clone(), gallery.as_weak());
        gallery.on_open_window(move |which| {
            let Some(gallery) = weak.upgrade() else { return };
            match open_window(&which, &gallery) {
                Ok(window) => opened.borrow_mut().push(window),
                Err(error) => log::error!("gallery: {which} window failed: {error:#}"),
            }
        });
    }
    // gpui reads the managed windows every frame; the page asks twice a second.
    let refresh = slint::Timer::default();
    {
        let (weak, count) = (gallery.as_weak(), Rc::new(RefCell::new(0)));
        refresh.start(slint::TimerMode::Repeated, std::time::Duration::from_millis(500), move || {
            let now = ely_slint::windows::WindowManager::windows().len();
            if now != *count.borrow() {
                *count.borrow_mut() = now;
                if let Some(gallery) = weak.upgrade() {
                    gallery.global::<ManagedWindows>().invoke_list();
                }
            }
        });
    }

    // The tray (system.rs `tray`) and the Dock badge (`dock`).
    let tray: Rc<RefCell<Option<Tray>>> = Rc::default();
    {
        let (tray, weak) = (tray.clone(), gallery.as_weak());
        gallery.on_tray_toggle(move || {
            let Some(gallery) = weak.upgrade() else { return };
            let shown = tray.borrow().is_some();
            if shown {
                tray.borrow_mut().take();
            } else {
                *tray.borrow_mut() = make_tray(&gallery);
            }
            gallery.set_tray_shown(tray.borrow().is_some());
        });
    }
    {
        let tray = tray.clone();
        gallery.on_tray_pick(move || {
            #[cfg(feature = "desktop")]
            if let Some(icon) = tray.borrow().as_ref() {
                icon.pick(0);
            }
            #[cfg(not(feature = "desktop"))]
            let _ = &tray;
        });
    }
    {
        let weak = gallery.as_weak();
        gallery.on_badge(move |label| {
            #[cfg(feature = "desktop")]
            if let Err(error) = ely_slint::desktop::set_dock_badge((!label.is_empty()).then_some(label.as_str())) {
                log::error!("gallery: badge failed: {error:#}");
            }
            #[cfg(not(feature = "desktop"))]
            let _ = label;
            if let Some(gallery) = weak.upgrade() {
                gallery.set_dock_reading(dock_reading().into());
            }
        });
    }
    Shell { _opened: opened, _refresh: refresh, _tray: tray }
}

#[cfg(feature = "desktop")]
fn make_tray(gallery: &Gallery) -> Option<Tray> {
    use ely_slint::desktop::{TrayIcon, TrayItem, TrayLook};
    let items = vec![
        TrayItem::Action(TRAY_ITEMS[0].into()),
        TrayItem::Action(TRAY_ITEMS[1].into()),
        TrayItem::Separator,
        TrayItem::Action(TRAY_ITEMS[2].into()),
    ];
    let look = TrayLook {
        symbol: "circle.hexagongrid",
        png: include_bytes!("../../../tests/assets/olive.png"),
        name: "Gallery".into(),
    };
    let weak = gallery.as_weak();
    let made = TrayIcon::new(look, items, move |index| {
        let label = match index {
            0 => TRAY_ITEMS[0],
            1 => TRAY_ITEMS[1],
            3 => TRAY_ITEMS[2],
            other => panic!("tray has no action at {other}"),
        };
        if let Some(gallery) = weak.upgrade() {
            gallery.set_tray_note(label.into());
        }
    });
    match made {
        Ok(icon) => Some(icon),
        Err(error) => {
            log::error!("gallery: tray failed: {error:#}");
            None
        }
    }
}

#[cfg(not(feature = "desktop"))]
fn make_tray(_: &Gallery) -> Option<Tray> {
    log::error!("gallery: tray failed: the tray needs the desktop feature");
    None
}

/// Opens one of the page's windows and wires it; the caller keeps it alive.
fn open_window(which: &str, gallery: &Gallery) -> anyhow::Result<Box<dyn Any>> {
    use ely_slint::window::{self, Corner};
    Ok(match which {
        "about" => {
            let about = AboutWindow::new()?;
            ely_slint::wire_text_format!(about);
            ely_slint::wire_links!(about);
            let _ = ely_slint::wire_window!(about);
            window::open_hosted(about.window(), "About Ely")?;
            Box::new(about)
        }
        "splash" => {
            let splash = SplashWindow::new()?;
            ely_slint::wire_text_format!(splash);
            let _ = ely_slint::wire_window!(splash);
            window::open_splash(splash.window())?;
            Box::new(splash)
        }
        "update" => {
            let update = UpdateWindow::new()?;
            let _ = ely_slint::wire_window!(update);
            update.on_logged(|what| log::info!("gallery: {what}"));
            window::open_hosted(update.window(), "Software Update")?;
            Box::new(update)
        }
        "crash" => {
            let crash = CrashWindow::new()?;
            let _ = ely_slint::wire_window!(crash);
            crash.on_logged(|what| log::info!("gallery: {what}"));
            window::open_hosted(crash.window(), "Ely crashed")?;
            Box::new(crash)
        }
        "managed" => {
            let number = ely_slint::windows::WindowManager::windows().len() + 1;
            let title = format!("Draft {number}");
            let draft = DraftWindow::new()?;
            let _ = ely_slint::wire_window!(draft);
            draft.set_heading(title.as_str().into());
            ely_slint::windows::WindowManager::open(&draft, &title)?;
            gallery.global::<ManagedWindows>().invoke_list();
            Box::new(draft)
        }
        "mini" => {
            let mini = MiniPlayer::new()?;
            let _ = ely_slint::wire_mini_window!(mini, Some(gallery.as_weak()));
            window::open_mini(mini.window(), "Now playing", Some(Corner::BottomRight))?;
            Box::new(mini)
        }
        "pip" => {
            let pip = PipWindow::new()?;
            let _ = ely_slint::wire_mini_window!(pip, Some(gallery.as_weak()));
            window::open_mini(pip.window(), "Picture in picture", Some(Corner::TopRight))?;
            Box::new(pip)
        }
        other => anyhow::bail!("no window called {other}"),
    })
}
