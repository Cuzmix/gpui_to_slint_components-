//! The model behind the `PaneGroup` component (ely-core/src/layout/panes.rs): editor-style
//! split panes, split right or down, closed, focused. The view reads it through the
//! `PaneGroups` global (`wire_workspace!`).

use anyhow::Context as _;
use serde::{Deserialize, Serialize};

use crate::layout::{check_weights, split_shares};

/// Names a pane; the first is 1.
pub type PaneId = u64;

/// The width of a split's handle: the handle's hit area, 6 px.
pub const HANDLE: f32 = 6.0;

/// Which way a split runs; saved as gpui's `Axis`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Axis {
    /// Stacked.
    Vertical,
    /// Side by side.
    Horizontal,
}

/// Split tree. The live model and its saved form are the same data.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum PaneLayout {
    /// One pane.
    Pane(PaneId),
    /// Panes side by side or stacked.
    Split {
        /// Horizontal sets them side by side.
        axis: Axis,
        /// Each child's share.
        sizes: Vec<f32>,
        /// The panes or splits it holds.
        children: Vec<PaneLayout>,
    },
}

impl PaneLayout {
    /// Checks ids and splits; returns the next free pane id.
    pub fn validate(&self) -> anyhow::Result<PaneId> {
        let mut ids = Vec::new();
        self.panes(&mut ids);
        let mut unique = ids.clone();
        unique.sort_unstable();
        unique.dedup();
        anyhow::ensure!(unique.len() == ids.len(), "pane ids repeat: {ids:?}");
        check_sizes(self)?;
        let last = *unique.last().expect("a layout holds a pane");
        last.checked_add(1).with_context(|| format!("no pane id follows {last}"))
    }

    fn panes(&self, out: &mut Vec<PaneId>) {
        match self {
            PaneLayout::Pane(id) => out.push(*id),
            PaneLayout::Split { children, .. } => children.iter().for_each(|child| child.panes(out)),
        }
    }

    /// Inserts `new` beside `target`; true if `target` was found.
    fn split(&mut self, target: PaneId, axis: Axis, new: PaneId) -> bool {
        match self {
            PaneLayout::Pane(id) if *id == target => {
                *self = PaneLayout::Split {
                    axis,
                    sizes: vec![0.5, 0.5],
                    children: vec![PaneLayout::Pane(target), PaneLayout::Pane(new)],
                };
                true
            }
            PaneLayout::Pane(_) => false,
            PaneLayout::Split { axis: own, sizes, children } => {
                let direct = children.iter().position(|child| *child == PaneLayout::Pane(target));
                match direct {
                    Some(ix) if *own == axis => {
                        let total: f32 = sizes.iter().sum();
                        sizes.iter_mut().for_each(|size| *size /= total);
                        let half = sizes[ix] / 2.0;
                        sizes[ix] = half;
                        sizes.insert(ix + 1, half);
                        children.insert(ix + 1, PaneLayout::Pane(new));
                        true
                    }
                    _ => children.iter_mut().any(|child| child.split(target, axis, new)),
                }
            }
        }
    }

    /// Removes `target`, folding splits left with one child.
    fn close(&mut self, target: PaneId) -> bool {
        let PaneLayout::Split { sizes, children, .. } = self else {
            return false;
        };
        if let Some(ix) = children.iter().position(|child| *child == PaneLayout::Pane(target)) {
            children.remove(ix);
            sizes.remove(ix);
            let rest: f32 = sizes.iter().sum();
            if rest > 0.0 {
                sizes.iter_mut().for_each(|size| *size /= rest);
            } else {
                sizes.iter_mut().for_each(|size| *size = 1.0);
            }
        } else if !children.iter_mut().any(|child| child.close(target)) {
            return false;
        }
        if children.len() == 1 {
            *self = children.remove(0);
        }
        true
    }

    /// Sets the split at `path`; false if the tree no longer has it.
    fn set_sizes(&mut self, path: &[usize], shares: &[f32]) -> bool {
        let PaneLayout::Split { sizes, children, .. } = self else {
            return false;
        };
        match path.split_first() {
            None if shares.len() == sizes.len() => {
                *sizes = shares.to_vec();
                true
            }
            None => false,
            Some((ix, rest)) => children.get_mut(*ix).is_some_and(|child| child.set_sizes(rest, shares)),
        }
    }

    fn at(&self, path: &[usize]) -> Option<&PaneLayout> {
        match path.split_first() {
            None => Some(self),
            Some((ix, rest)) => match self {
                PaneLayout::Split { children, .. } => children.get(*ix)?.at(rest),
                PaneLayout::Pane(_) => None,
            },
        }
    }
}

fn check_sizes(node: &PaneLayout) -> anyhow::Result<()> {
    if let PaneLayout::Split { sizes, children, .. } = node {
        anyhow::ensure!(children.len() >= 2, "a split needs two children");
        anyhow::ensure!(sizes.len() == children.len(), "split sizes {sizes:?} miss children");
        check_weights(sizes).map_err(anyhow::Error::msg)?;
        children.iter().try_for_each(check_sizes)?;
    }
    Ok(())
}

/// A length that follows the group's size: `scale` × the group's width (or height) + `offset`
/// pixels. Every edge of a split tree is one, since the handles keep 6 px and the cells share
/// the rest.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Span {
    /// Of the group's whole length.
    pub scale: f32,
    /// Pixels.
    pub offset: f32,
}

impl Span {
    const ZERO: Span = Span { scale: 0.0, offset: 0.0 };
    const WHOLE: Span = Span { scale: 1.0, offset: 0.0 };

    /// Its length in a group `whole` pixels long.
    pub fn at(self, whole: f32) -> f32 {
        self.scale * whole + self.offset
    }

    fn plus(self, other: Span) -> Span {
        Span { scale: self.scale + other.scale, offset: self.offset + other.offset }
    }

    fn minus(self, other: Span) -> Span {
        Span { scale: self.scale - other.scale, offset: self.offset - other.offset }
    }

    fn times(self, factor: f32) -> Span {
        Span { scale: self.scale * factor, offset: self.offset * factor }
    }

    fn px(offset: f32) -> Span {
        Span { scale: 0.0, offset }
    }
}

/// A box's four edges in the group's coordinates.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Edges {
    /// Of the group's width.
    pub left: Span,
    /// Of the group's width.
    pub right: Span,
    /// Of the group's height.
    pub top: Span,
    /// Of the group's height.
    pub bottom: Span,
}

/// A box in the group's own coordinates, in pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rect {
    /// From the group's left edge.
    pub x: f32,
    /// From the group's top edge.
    pub y: f32,
    /// Across.
    pub width: f32,
    /// Down.
    pub height: f32,
}

impl Edges {
    /// The box in a group `width` × `height`, each edge on a whole pixel as taffy places it.
    pub fn rect(&self, width: f32, height: f32) -> Rect {
        let (left, right) = (self.left.at(width).round(), self.right.at(width).round());
        let (top, bottom) = (self.top.at(height).round(), self.bottom.at(height).round());
        Rect { x: left, y: top, width: (right - left).max(0.0), height: (bottom - top).max(0.0) }
    }
}

/// Where a pane is drawn.
#[derive(Clone, Debug, PartialEq)]
pub struct PanePlace {
    /// The pane.
    pub id: PaneId,
    /// Its box, header included.
    pub edges: Edges,
}

/// A split's divider: between children `index` and `index + 1` of the split at `path`.
#[derive(Clone, Debug, PartialEq)]
pub struct PaneHandle {
    /// The split, as child indexes from the root.
    pub path: Vec<usize>,
    /// The divider's place in that split.
    pub index: usize,
    /// The split's direction.
    pub axis: Axis,
    /// The handle's hit box.
    pub edges: Edges,
    /// The split's length along its axis, handles included, of the group's length that way.
    pub extent: Span,
}

/// Lays out a split the way taffy does gpui's flex: the handles keep 6 px and the cells share
/// the rest by weight. (A split shorter than its handles gets negative room here, where taffy
/// would give its cells nothing; the boxes come out empty either way.)
fn place(node: &PaneLayout, edges: Edges, path: &mut Vec<usize>, panes: &mut Vec<PanePlace>, handles: &mut Vec<PaneHandle>) {
    match node {
        PaneLayout::Pane(id) => panes.push(PanePlace { id: *id, edges }),
        PaneLayout::Split { axis, sizes, children } => {
            let count = children.len();
            let shares = split_shares(sizes, count);
            let horizontal = *axis == Axis::Horizontal;
            let (start, end) = if horizontal { (edges.left, edges.right) } else { (edges.top, edges.bottom) };
            let extent = end.minus(start);
            let room = extent.minus(Span::px(HANDLE * count.saturating_sub(1) as f32));
            let along = |from: Span, to: Span| {
                if horizontal { Edges { left: from, right: to, ..edges } } else { Edges { top: from, bottom: to, ..edges } }
            };
            let mut at = start;
            for (ix, child) in children.iter().enumerate() {
                if ix > 0 {
                    let next = at.plus(Span::px(HANDLE));
                    handles.push(PaneHandle { path: path.clone(), index: ix - 1, axis: *axis, edges: along(at, next), extent });
                    at = next;
                }
                let next = at.plus(room.times(shares.get(ix).copied().unwrap_or(0.0)));
                path.push(ix);
                place(child, along(at, next), path, panes, handles);
                path.pop();
                at = next;
            }
        }
    }
}

/// A divider drag under way: the split, the divider, the shares and length it began with.
#[derive(Clone, Debug)]
struct Grab {
    path: Vec<usize>,
    index: usize,
    start: Vec<f32>,
    extent: f32,
}

/// Editor-style split panes: split right or down, close, focus.
///
/// ```
/// use ely_slint::panes::{Axis, PaneGroup};
///
/// let mut group = PaneGroup::new();
/// assert_eq!(group.split(1, Axis::Horizontal), Some(2));
/// assert_eq!((group.panes(), group.focused()), (vec![1, 2], 2));
/// ```
#[derive(Clone, Debug)]
pub struct PaneGroup {
    root: PaneLayout,
    focused: PaneId,
    next: PaneId,
    grabbed: Option<Grab>,
}

impl Default for PaneGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl PaneGroup {
    /// One pane, 1.
    pub fn new() -> Self {
        Self { root: PaneLayout::Pane(1), focused: 1, next: 2, grabbed: None }
    }

    /// The focused pane.
    pub fn focused(&self) -> PaneId {
        self.focused
    }

    /// Every pane, in tree order.
    pub fn panes(&self) -> Vec<PaneId> {
        let mut out = Vec::new();
        self.root.panes(&mut out);
        out
    }

    /// Opens a new pane beside `pane` and focuses it; `None` if `pane` is gone.
    pub fn split(&mut self, pane: PaneId, axis: Axis) -> Option<PaneId> {
        let new = self.next;
        if !self.root.split(pane, axis, new) {
            log::error!("pane group: no pane {pane} to split");
            return None;
        }
        self.next = new.checked_add(1).expect("pane ids exhausted");
        self.focused = new;
        log::info!("pane group: split {pane} -> {new} ({axis:?})");
        Some(new)
    }

    /// Closes `pane`; the last pane, or one already gone, stays as it is.
    pub fn close(&mut self, pane: PaneId) {
        if self.panes().len() < 2 {
            log::error!("pane group: pane {pane} is the last; it stays");
            return;
        }
        if !self.root.close(pane) {
            log::error!("pane group: no pane {pane} to close");
            return;
        }
        if self.focused == pane {
            self.focused = self.panes()[0];
        }
        log::info!("pane group: closed {pane}");
    }

    /// Focuses `pane`; an unknown one logs and changes nothing.
    pub fn focus(&mut self, pane: PaneId) {
        if !self.panes().contains(&pane) {
            log::error!("pane group: no pane {pane} to focus");
            return;
        }
        self.focused = pane;
    }

    /// The tree to save.
    pub fn layout(&self) -> PaneLayout {
        self.root.clone()
    }

    /// Replaces the tree after `PaneLayout::validate` accepts it.
    pub fn restore(&mut self, layout: PaneLayout) -> anyhow::Result<()> {
        self.next = layout.validate()?;
        let mut ids = Vec::new();
        layout.panes(&mut ids);
        self.focused = ids[0];
        self.root = layout;
        self.grabbed = None;
        log::info!("pane group: restored {} panes", ids.len());
        Ok(())
    }

    /// The panes' and handles' boxes, for any size of group.
    pub fn geometry(&self) -> (Vec<PanePlace>, Vec<PaneHandle>) {
        let (mut panes, mut handles) = (Vec::new(), Vec::new());
        let whole = Edges { left: Span::ZERO, right: Span::WHOLE, top: Span::ZERO, bottom: Span::WHOLE };
        place(&self.root, whole, &mut Vec::new(), &mut panes, &mut handles);
        (panes, handles)
    }

    /// A drag of `handle` began in a group `width` × `height`: its split's shares now are where
    /// the drag measures from.
    pub fn grab(&mut self, handle: &PaneHandle, width: f32, height: f32) {
        let Some(PaneLayout::Split { sizes, children, .. }) = self.root.at(&handle.path) else {
            log::error!("pane group: no split at {:?} to grab", handle.path);
            self.grabbed = None;
            return;
        };
        let shares = split_shares(sizes, children.len());
        self.grabbed = Some(Grab { path: handle.path.clone(), index: handle.index, start: shares, extent: handle.extent.at(if handle.axis == Axis::Horizontal { width } else { height }) });
    }

    /// The grabbed handle is `delta` pixels from where its drag first moved; no pane of its
    /// split gets under a pane's least size, 160 px.
    pub fn drag(&mut self, delta: f32) {
        let Some(Grab { path, index, start, extent }) = &self.grabbed else {
            return;
        };
        if *extent <= 0.0 {
            return;
        }
        let next = crate::layout::split_shift(start, *index, delta / extent, 160.0 / extent);
        if next.len() != start.len() {
            return;
        }
        if !self.root.set_sizes(path, &next) {
            log::error!("pane group: no split at {path:?} for {next:?}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ids(layout: &PaneLayout) -> Vec<u64> {
        let mut out = Vec::new();
        layout.panes(&mut out);
        out
    }

    #[test]
    fn split_nests_or_extends() {
        let mut root = PaneLayout::Pane(1);
        assert!(root.split(1, Axis::Horizontal, 2));
        assert!(root.split(2, Axis::Horizontal, 3));
        let PaneLayout::Split { sizes, .. } = &root else { panic!("split") };
        assert_eq!(sizes, &[0.5, 0.25, 0.25]);
        assert!(root.split(3, Axis::Vertical, 4));
        assert_eq!(ids(&root), [1, 2, 3, 4]);
        assert!(!root.split(9, Axis::Vertical, 5));
        let mut tiny = PaneLayout::Split {
            axis: Axis::Horizontal,
            sizes: vec![1e-45, 0.0],
            children: vec![PaneLayout::Pane(1), PaneLayout::Pane(2)],
        };
        assert!(tiny.split(1, Axis::Horizontal, 3));
        assert!(tiny.validate().is_ok());
    }

    #[test]
    fn validate_refuses_bad_weights_and_exhausted_ids() {
        let split = |sizes: Vec<f32>| PaneLayout::Split {
            axis: Axis::Horizontal,
            sizes,
            children: vec![PaneLayout::Pane(1), PaneLayout::Pane(2)],
        };
        assert_eq!(split(vec![1.0, 3.0]).validate().unwrap(), 3);
        assert!(split(vec![0.0, 0.0]).validate().is_err());
        assert!(split(vec![-1.0, 2.0]).validate().is_err());
        assert!(split(vec![f32::MAX, f32::MAX]).validate().is_err());
        assert!(PaneLayout::Pane(u64::MAX).validate().is_err());
    }

    #[test]
    fn close_keeps_weights_finite() {
        let three = |sizes: Vec<f32>| PaneLayout::Split {
            axis: Axis::Horizontal,
            sizes,
            children: vec![PaneLayout::Pane(1), PaneLayout::Pane(2), PaneLayout::Pane(3)],
        };
        let sizes = |layout: &PaneLayout| match layout {
            PaneLayout::Split { sizes, .. } => sizes.clone(),
            PaneLayout::Pane(_) => panic!("split"),
        };
        let mut zeros = three(vec![0.0, 0.0, 1.0]);
        assert!(zeros.close(3));
        assert_eq!(sizes(&zeros), [1.0, 1.0]);
        let mut tiny = three(vec![0.0, 0.0, 1e-45]);
        assert!(tiny.close(3));
        assert!(tiny.validate().is_ok());
        let mut huge = three(vec![1e30, 1e30, 1e30]);
        assert!(huge.close(1));
        assert_eq!(sizes(&huge), [0.5, 0.5]);
        let mut kept = three(vec![1.0, 3.0, 4.0]);
        assert!(kept.close(3));
        assert_eq!(sizes(&kept), [0.25, 0.75]);
    }

    #[test]
    fn a_stale_split_path_sets_nothing() {
        let mut root = PaneLayout::Pane(1);
        root.split(1, Axis::Horizontal, 2);
        assert!(!root.set_sizes(&[0], &[0.5, 0.5]), "a leaf");
        assert!(!root.set_sizes(&[4], &[0.5, 0.5]), "past the children");
        assert!(!root.set_sizes(&[], &[1.0]), "a share too few");
        assert!(root.set_sizes(&[], &[0.3, 0.7]));
    }

    #[test]
    fn close_folds_single_children() {
        let mut root = PaneLayout::Pane(1);
        root.split(1, Axis::Horizontal, 2);
        root.split(2, Axis::Vertical, 3);
        assert!(root.close(3));
        assert_eq!(ids(&root), [1, 2]);
        assert!(root.close(2));
        assert_eq!(root, PaneLayout::Pane(1));
    }

    // ely-core/src/layout/tests.rs `a_pane_group_leaves_a_gone_or_last_pane_as_it_is`.
    #[test]
    fn a_pane_group_leaves_a_gone_or_last_pane_as_it_is() {
        let mut group = PaneGroup::new();
        assert_eq!(group.split(9, Axis::Horizontal), None);
        group.close(1);
        assert_eq!(group.panes(), [1], "the last pane stays");
        let second = group.split(1, Axis::Horizontal).expect("pane 1 splits");
        group.close(second);
        group.close(second);
        group.focus(second);
        assert_eq!((group.panes(), group.focused()), (vec![1], 1));
    }

    #[test]
    fn geometry_keeps_handles_and_rounds_edges() {
        let mut group = PaneGroup::new();
        group.split(1, Axis::Horizontal);
        group.split(2, Axis::Vertical);
        let (panes, handles) = group.geometry();
        let rect = |x, y, width, height| Rect { x, y, width, height };
        // (761 - 6) / 2 = 377.5 each: edges at 0, 377.5 → 378, 383.5 → 384, 761.
        assert_eq!((panes[0].id, panes[0].edges.rect(761.0, 300.0)), (1, rect(0.0, 0.0, 378.0, 300.0)));
        assert_eq!(handles[0].edges.rect(761.0, 300.0), rect(378.0, 0.0, 6.0, 300.0));
        assert_eq!(panes[1].edges.rect(761.0, 300.0), rect(384.0, 0.0, 377.0, 147.0));
        assert_eq!(handles[1].edges.rect(761.0, 300.0), rect(384.0, 147.0, 377.0, 6.0));
        assert_eq!(panes[2].edges.rect(761.0, 300.0), rect(384.0, 153.0, 377.0, 147.0));
        assert_eq!((handles[1].path.clone(), handles[1].extent.at(300.0)), (vec![1], 300.0));
        assert_eq!(handles[0].extent.at(761.0), 761.0);
    }

    #[test]
    fn a_drag_moves_one_divider_and_keeps_the_floor() {
        let mut group = PaneGroup::new();
        group.split(1, Axis::Horizontal);
        let (_, handles) = group.geometry();
        group.grab(&handles[0], 606.0, 300.0);
        group.drag(60.0);
        let PaneLayout::Split { sizes, .. } = group.layout() else { panic!("split") };
        assert!((sizes[0] - (0.5 + 60.0 / 606.0)).abs() < 1e-6);
        group.drag(-600.0);
        let PaneLayout::Split { sizes, .. } = group.layout() else { panic!("split") };
        assert!((sizes[0] - 160.0 / 606.0).abs() < 1e-6, "{sizes:?}");
    }
}
