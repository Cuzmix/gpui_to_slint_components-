//! Saved window layouts (ely-core/src/layout/workspace.rs), and the host that keeps the docks
//! and pane groups the `Dock` and `PaneGroup` components draw (`wire_workspace!`).

use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use anyhow::Context as _;
use serde::{Deserialize, Serialize};

use crate::{
    dock::{Dock, DockLayout},
    panes::{PaneGroup, PaneLayout},
};

/// Bump when a field changes meaning; never reuse a removed name.
pub const WORKSPACE_VERSION: u32 = 1;

/// Saved window layout: docks and editor panes, as JSON.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Workspace {
    /// The format's version.
    pub version: u32,
    /// The docks.
    pub dock: DockLayout,
    /// The editor panes.
    pub panes: PaneLayout,
}

impl Workspace {
    /// The layout `dock` and `panes` hold now.
    pub fn capture(dock: &Dock, panes: &PaneGroup) -> Self {
        Self { version: WORKSPACE_VERSION, dock: dock.layout(), panes: panes.layout() }
    }

    /// Pretty JSON.
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("workspace layout always serializes")
    }

    /// Unknown fields are ignored; a newer version is refused.
    pub fn from_json(json: &str) -> anyhow::Result<Self> {
        let workspace: Self = serde_json::from_str(json).context("workspace layout json")?;
        anyhow::ensure!(
            workspace.version <= WORKSPACE_VERSION,
            "workspace version {} is newer than this build ({WORKSPACE_VERSION})",
            workspace.version
        );
        Ok(workspace)
    }

    /// Applies all of it or none of it.
    pub fn apply(&self, dock: &mut Dock, panes: &mut PaneGroup) -> anyhow::Result<()> {
        self.panes.validate()?;
        dock.restore(&self.dock)?;
        panes.restore(self.panes.clone())
    }
}

/// The docks and pane groups a window draws, each by the name its component gives.
#[derive(Debug, Default)]
pub struct Systems {
    /// `Dock { name: ... }`'s models.
    pub docks: BTreeMap<String, Dock>,
    /// `PaneGroup { name: ... }`'s models.
    pub groups: BTreeMap<String, PaneGroup>,
}

impl Systems {
    /// The layout of dock `dock` and pane group `panes`; `None` if either is missing.
    pub fn capture(&self, dock: &str, panes: &str) -> Option<Workspace> {
        Some(Workspace::capture(self.docks.get(dock)?, self.groups.get(panes)?))
    }

    /// Applies `layout` to dock `dock` and pane group `panes`, all or nothing.
    pub fn apply(&mut self, layout: &Workspace, dock: &str, panes: &str) -> anyhow::Result<()> {
        let docked = self.docks.get_mut(dock).with_context(|| format!("no dock {dock}"))?;
        let group = self.groups.get_mut(panes).with_context(|| format!("no pane group {panes}"))?;
        layout.apply(docked, group)
    }
}

/// What `wire_workspace!` returns: the window's docks and pane groups. Change them with
/// `update`, which redraws the components after.
pub struct WorkspaceHost {
    systems: RefCell<Systems>,
    redraw: RefCell<Option<Box<dyn Fn(&Systems)>>>,
}

impl WorkspaceHost {
    #[doc(hidden)]
    pub fn new() -> Rc<Self> {
        Rc::new(Self { systems: RefCell::default(), redraw: RefCell::new(None) })
    }

    #[doc(hidden)]
    pub fn on_redraw(&self, redraw: impl Fn(&Systems) + 'static) {
        *self.redraw.borrow_mut() = Some(Box::new(redraw));
    }

    /// Changes the docks and pane groups, then shows the change.
    pub fn update<R>(&self, change: impl FnOnce(&mut Systems) -> R) -> R {
        let out = change(&mut self.systems.borrow_mut());
        self.redraw();
        out
    }

    /// Reads them.
    pub fn read<R>(&self, look: impl FnOnce(&Systems) -> R) -> R {
        look(&self.systems.borrow())
    }

    /// Changes them without drawing again, as gpui does for a floating panel's new corner and
    /// a split's dragged shares.
    pub fn quietly<R>(&self, change: impl FnOnce(&mut Systems) -> R) -> R {
        change(&mut self.systems.borrow_mut())
    }

    #[doc(hidden)]
    pub fn redraw(&self) {
        if let Some(redraw) = self.redraw.borrow().as_ref() {
            redraw(&self.systems.borrow());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        dock::{DockPanel, DockSide, Spot},
        panes::Axis,
    };

    fn sample() -> Workspace {
        Workspace {
            version: WORKSPACE_VERSION,
            dock: DockLayout {
                spots: vec![
                    ("files".into(), Spot::Docked(DockSide::Left)),
                    ("outline".into(), Spot::Floating { x: 40.0, y: 60.0 }),
                ],
                active: [Some("files".into()), None, None, None],
                sizes: [15.0, 17.5, 12.0, 13.75],
            },
            panes: PaneLayout::Split {
                axis: Axis::Horizontal,
                sizes: vec![0.5, 0.5],
                children: vec![PaneLayout::Pane(1), PaneLayout::Pane(2)],
            },
        }
    }

    #[test]
    fn round_trips_through_json() {
        let saved = sample();
        assert_eq!(Workspace::from_json(&saved.to_json()).unwrap(), saved);
    }

    #[test]
    fn ignores_unknown_fields_but_refuses_newer_versions() {
        let mut value = serde_json::to_value(sample()).unwrap();
        value["added_later"] = serde_json::json!(true);
        assert!(Workspace::from_json(&value.to_string()).is_ok());
        value["version"] = serde_json::json!(WORKSPACE_VERSION + 1);
        assert!(Workspace::from_json(&value.to_string()).is_err());
    }

    #[test]
    fn invalid_pane_trees_are_refused() {
        let bad = PaneLayout::Split {
            axis: Axis::Vertical,
            sizes: vec![1.0],
            children: vec![PaneLayout::Pane(1), PaneLayout::Pane(1)],
        };
        assert!(bad.validate().is_err());
    }

    #[test]
    fn the_json_spells_spots_and_splits_as_gpui_does() {
        let json = serde_json::to_value(sample()).unwrap();
        assert_eq!(json["dock"]["spots"][0], serde_json::json!(["files", { "Docked": "Left" }]));
        assert_eq!(json["dock"]["spots"][1][1], serde_json::json!({ "Floating": { "x": 40.0, "y": 60.0 } }));
        assert_eq!(json["panes"]["Split"]["axis"], "Horizontal");
        assert_eq!(json["panes"]["Split"]["children"][0], serde_json::json!({ "Pane": 1 }));
    }

    #[test]
    fn a_refused_layout_leaves_both_as_they_were() {
        let mut dock = Dock::new();
        let panel = |id: &str| DockPanel { id: id.into(), title: id.into(), icon: "folder".into() };
        dock.add(panel("files"), DockSide::Left);
        dock.add(panel("outline"), DockSide::Left);
        let mut panes = PaneGroup::new();
        let mut bad = sample();
        bad.panes = PaneLayout::Split {
            axis: Axis::Vertical,
            sizes: vec![0.0, 0.0],
            children: vec![PaneLayout::Pane(1), PaneLayout::Pane(2)],
        };
        let (dock_before, panes_before) = (dock.layout(), panes.layout());
        assert!(bad.apply(&mut dock, &mut panes).is_err());
        assert_eq!((dock.layout(), panes.layout()), (dock_before, panes_before));
        sample().apply(&mut dock, &mut panes).unwrap();
        assert_eq!(panes.panes(), [1, 2]);
        assert_eq!(dock.floating().len(), 1);
    }
}
