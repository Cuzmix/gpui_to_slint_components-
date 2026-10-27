//! The model behind the `Dock` component (ely-core/src/layout/dock/mod.rs): panels on the four
//! edges, tabbed per edge, or floating. The view reads it through the `Docks` global
//! (`wire_workspace!`).

use serde::{Deserialize, Serialize};

/// gpui's rem in this port: sizes are saved in rems of 16 px.
pub const REM: f32 = 16.0;

/// A window edge a dock sits on.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DockSide {
    /// The left edge.
    Left,
    /// The right edge.
    Right,
    /// The top edge.
    Top,
    /// The bottom edge.
    Bottom,
}

impl DockSide {
    /// Every side, in the order layouts store them.
    pub const ALL: [DockSide; 4] = [DockSide::Left, DockSide::Right, DockSide::Top, DockSide::Bottom];

    fn index(self) -> usize {
        self as usize
    }
}

/// Where a panel lives: a dock, or floating at a window position.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Spot {
    /// In the dock on that side.
    Docked(DockSide),
    /// Floating, its top-left corner in window pixels.
    Floating {
        /// From the window's left edge.
        x: f32,
        /// From the window's top edge.
        y: f32,
    },
}

/// A panel a dock holds.
#[derive(Clone, Debug)]
pub struct DockPanel {
    /// Names it in the layout, and to `activate` and `move_panel`.
    pub id: String,
    /// Shown on its tab.
    pub title: String,
    /// Shown beside its title, by its name in `Icons.all` (`folder`, `search`, ...).
    pub icon: String,
}

/// Saved dock arrangement, per side in `DockSide` order. Sizes are in rems.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DockLayout {
    /// Each panel's id and where it sits.
    pub spots: Vec<(String, Spot)>,
    /// Each side's shown panel, by id.
    pub active: [Option<String>; 4],
    /// Each side's size.
    pub sizes: [f32; 4],
}

/// Side docks around a center; panels move between docks and float.
///
/// ```
/// use ely_slint::dock::{Dock, DockPanel, DockSide};
///
/// let mut dock = Dock::new();
/// dock.add(DockPanel { id: "files".into(), title: "Files".into(), icon: "folder".into() }, DockSide::Left);
/// assert_eq!(dock.active(DockSide::Left).map(|panel| panel.id.as_str()), Some("files"));
/// ```
#[derive(Clone, Debug)]
pub struct Dock {
    panels: Vec<DockPanel>,
    spots: Vec<Spot>,
    active: [Option<usize>; 4],
    sizes: [f32; 4],
}

impl Default for Dock {
    fn default() -> Self {
        Self::new()
    }
}

impl Dock {
    /// An empty dock: the sides 240 px (an open sidebar), the top and bottom 160 px (a pane's
    /// least size).
    pub fn new() -> Self {
        let (side, strip) = (240.0 / REM, 160.0 / REM);
        Self {
            panels: Vec::new(),
            spots: Vec::new(),
            active: [None; 4],
            sizes: [side, side, strip, strip],
        }
    }

    /// Panics on a repeated id.
    pub fn add(&mut self, panel: DockPanel, side: DockSide) {
        assert!(self.find(&panel.id).is_none(), "dock panel {} added twice", panel.id);
        self.panels.push(panel);
        self.spots.push(Spot::Docked(side));
        let slot = &mut self.active[side.index()];
        if slot.is_none() {
            *slot = Some(self.panels.len() - 1);
        }
    }

    /// The panel with this id, if any.
    pub fn find(&self, id: &str) -> Option<usize> {
        self.panels.iter().position(|panel| panel.id == id)
    }

    fn on_side(&self, side: DockSide) -> Vec<usize> {
        (0..self.panels.len()).filter(|ix| self.spots[*ix] == Spot::Docked(side)).collect()
    }

    /// The panels docked on `side`, in the order they were added, each with whether its tab is
    /// the shown one.
    pub fn tabs(&self, side: DockSide) -> Vec<(&DockPanel, bool)> {
        let active = self.active[side.index()];
        self.on_side(side).into_iter().map(|ix| (&self.panels[ix], active == Some(ix))).collect()
    }

    /// The panel `side` shows; a side without one is not drawn.
    pub fn active(&self, side: DockSide) -> Option<&DockPanel> {
        self.active[side.index()].map(|ix| &self.panels[ix])
    }

    /// `side`'s size in pixels.
    pub fn size(&self, side: DockSide) -> f32 {
        self.sizes[side.index()] * REM
    }

    /// The floating panels and their corners, in the order they were added.
    pub fn floating(&self) -> Vec<(&DockPanel, f32, f32)> {
        self.panels
            .iter()
            .zip(&self.spots)
            .filter_map(|(panel, spot)| match spot {
                Spot::Floating { x, y } => Some((panel, *x, *y)),
                Spot::Docked(_) => None,
            })
            .collect()
    }

    /// Shows the panel `id` in its dock. Panics on an unknown id.
    pub fn activate(&mut self, id: &str) {
        let ix = self.find(id).unwrap_or_else(|| panic!("no dock panel {id}"));
        if let Spot::Docked(side) = self.spots[ix] {
            self.active[side.index()] = Some(ix);
        }
    }

    /// Moves the panel `id` to `spot`. Panics on an unknown id.
    pub fn move_panel(&mut self, id: &str, spot: Spot) {
        let ix = self.find(id).unwrap_or_else(|| panic!("no dock panel {id}"));
        let from = self.spots[ix];
        self.spots[ix] = spot;
        if let Spot::Docked(side) = from
            && self.active[side.index()] == Some(ix)
        {
            self.active[side.index()] = self.on_side(side).first().copied();
        }
        if let Spot::Docked(side) = spot {
            self.active[side.index()] = Some(ix);
        }
        log::info!("dock: {id} {from:?} -> {spot:?}");
    }

    /// A floating panel's title bar was dragged to (`x`, `y`). gpui keeps the spot without
    /// drawing again; so does this. Panics on an unknown id.
    pub fn floated(&mut self, id: &str, x: f32, y: f32) {
        let ix = self.find(id).unwrap_or_else(|| panic!("no dock panel {id}"));
        self.spots[ix] = Spot::Floating { x, y };
    }

    /// A side's resize handle was dragged `span` pixels from that edge of the dock, which is
    /// `extent` pixels across that way: at least a pane's least size, at most 45 % of the dock.
    pub fn resize(&mut self, side: DockSide, span: f32, extent: f32) {
        let size = span.max(160.0).min(extent * 0.45);
        self.sizes[side.index()] = size / REM;
    }

    /// The arrangement to save.
    pub fn layout(&self) -> DockLayout {
        let name = |ix: Option<usize>| ix.map(|ix| self.panels[ix].id.clone());
        DockLayout {
            spots: self.panels.iter().zip(&self.spots).map(|(panel, spot)| (panel.id.clone(), *spot)).collect(),
            active: DockSide::ALL.map(|side| name(self.active[side.index()])),
            sizes: self.sizes,
        }
    }

    /// All or nothing. A side the layout leaves unset shows its first panel.
    pub fn restore(&mut self, layout: &DockLayout) -> anyhow::Result<()> {
        let mut named: Vec<&str> = layout.spots.iter().map(|(id, _)| id.as_str()).collect();
        named.sort_unstable();
        named.dedup();
        anyhow::ensure!(named.len() == layout.spots.len(), "layout places a panel twice");
        anyhow::ensure!(
            layout.sizes.iter().all(|size| size.is_finite() && *size > 0.0),
            "dock sizes {:?} must be finite and positive",
            layout.sizes
        );
        anyhow::ensure!(
            layout.spots.iter().all(|(_, spot)| match spot {
                Spot::Docked(_) => true,
                Spot::Floating { x, y } => x.is_finite() && y.is_finite(),
            }),
            "floating positions must be finite"
        );
        let unknown: Vec<&str> = layout
            .spots
            .iter()
            .map(|(id, _)| id.as_str())
            .chain(layout.active.iter().flatten().map(String::as_str))
            .filter(|id| self.find(id).is_none())
            .collect();
        anyhow::ensure!(unknown.is_empty(), "layout names unknown panels: {unknown:?}");
        let mut spots = self.spots.clone();
        for (id, spot) in &layout.spots {
            spots[self.find(id).expect("checked above")] = *spot;
        }
        let mut active = [None; 4];
        for side in DockSide::ALL {
            let chosen = layout.active[side.index()].as_deref().and_then(|id| self.find(id));
            if let Some(ix) = chosen {
                anyhow::ensure!(
                    spots[ix] == Spot::Docked(side),
                    "active panel {} is not docked {side:?}",
                    self.panels[ix].id
                );
            }
            active[side.index()] = chosen.or_else(|| (0..spots.len()).find(|ix| spots[*ix] == Spot::Docked(side)));
        }
        self.spots = spots;
        self.active = active;
        self.sizes = layout.sizes;
        log::info!("dock: restored {} panel spots", layout.spots.len());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn panel(id: &str) -> DockPanel {
        DockPanel { id: id.into(), title: id.into(), icon: "folder".into() }
    }

    fn demo() -> Dock {
        let mut dock = Dock::new();
        dock.add(panel("files"), DockSide::Left);
        dock.add(panel("outline"), DockSide::Left);
        dock.add(panel("search"), DockSide::Right);
        dock.add(panel("terminal"), DockSide::Bottom);
        dock
    }

    fn active(dock: &Dock, side: DockSide) -> Option<&str> {
        dock.active(side).map(|panel| panel.id.as_str())
    }

    #[test]
    fn the_first_panel_on_a_side_shows() {
        let dock = demo();
        assert_eq!(active(&dock, DockSide::Left), Some("files"));
        assert_eq!(active(&dock, DockSide::Top), None);
        assert_eq!(dock.tabs(DockSide::Left).iter().map(|(p, on)| (p.id.as_str(), *on)).collect::<Vec<_>>(), [("files", true), ("outline", false)]);
        assert_eq!(dock.size(DockSide::Left), 240.0);
        assert_eq!(dock.size(DockSide::Bottom), 160.0);
    }

    #[test]
    #[should_panic(expected = "added twice")]
    fn a_repeated_id_panics() {
        let mut dock = demo();
        dock.add(panel("files"), DockSide::Right);
    }

    #[test]
    fn moving_the_shown_panel_shows_the_next_and_the_moved_one() {
        let mut dock = demo();
        dock.move_panel("files", Spot::Docked(DockSide::Top));
        assert_eq!(active(&dock, DockSide::Left), Some("outline"));
        assert_eq!(active(&dock, DockSide::Top), Some("files"));
        dock.move_panel("outline", Spot::Floating { x: 40.0, y: 60.0 });
        assert_eq!(active(&dock, DockSide::Left), None);
        assert_eq!(dock.floating().len(), 1);
        dock.activate("outline");
        assert_eq!(active(&dock, DockSide::Left), None, "a floating panel shows on no side");
    }

    #[test]
    fn resizing_keeps_between_the_floor_and_45_percent() {
        let mut dock = demo();
        dock.resize(DockSide::Left, 20.0, 1000.0);
        assert_eq!(dock.size(DockSide::Left), 160.0);
        dock.resize(DockSide::Left, 900.0, 1000.0);
        assert_eq!(dock.size(DockSide::Left), 450.0);
        dock.resize(DockSide::Top, 300.0, 200.0);
        assert_eq!(dock.size(DockSide::Top), 90.0, "the cap wins over the floor");
        assert_eq!(dock.layout().sizes[2], 90.0 / REM);
    }

    #[test]
    fn restore_is_all_or_nothing() {
        let mut dock = demo();
        let saved = dock.layout();
        dock.move_panel("search", Spot::Docked(DockSide::Left));
        let moved = dock.layout();
        let mut twice = moved.clone();
        twice.spots.push(("files".into(), Spot::Docked(DockSide::Right)));
        assert!(dock.restore(&twice).is_err());
        let mut unknown = moved.clone();
        unknown.active[1] = Some("nope".into());
        assert!(dock.restore(&unknown).is_err());
        let mut sizes = moved.clone();
        sizes.sizes[0] = 0.0;
        assert!(dock.restore(&sizes).is_err());
        let mut nan = moved.clone();
        nan.spots[0].1 = Spot::Floating { x: f32::NAN, y: 0.0 };
        assert!(dock.restore(&nan).is_err());
        let mut misplaced = moved.clone();
        misplaced.active[0] = Some("terminal".into());
        assert!(dock.restore(&misplaced).is_err());
        assert_eq!(dock.layout(), moved, "a refused layout changes nothing");
        dock.restore(&saved).unwrap();
        assert_eq!(dock.layout(), saved);
        let mut unset = saved.clone();
        unset.active = [None, None, None, None];
        dock.restore(&unset).unwrap();
        assert_eq!(active(&dock, DockSide::Left), Some("files"), "an unset side shows its first panel");
    }
}
