//! What the gallery binary and the capture example both hand the gallery: the Rust globals, and
//! the layout chapter's dock, pane group and workspace (ely-gallery layout/systems.rs).

use std::{cell::RefCell, rc::Rc};

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
    Wired { _clock: clock, _host: host }
}
