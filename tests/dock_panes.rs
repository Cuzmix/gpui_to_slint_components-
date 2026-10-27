#[macro_use]
mod common;
use common::*;

use ely_slint::{
    dock::{Dock as DockModel, DockPanel, DockSide as Side, Spot},
    panes::{Axis, PaneGroup as PaneModel},
    workspace::{WorkspaceHost, Workspace},
};
use i_slint_backend_testing::ElementHandle;
use std::rc::Rc;

slint::slint! {
    import {
        Dock, DockFloat, DockSide, DockTab, DockEdge, DockFloatSpot, Docks,
        PaneGroup, PaneSpan, PaneCell, PaneDivider, PaneGroups, Icons, IconEntry,
    } from "ui/index.slint";
    export { Docks, DockSide, DockTab, DockEdge, DockFloatSpot, PaneGroups, PaneSpan, PaneCell, PaneDivider, Icons, IconEntry }
    export component Harness inherits Window {
        width: 800px;
        height: 600px;
        out property <string> shown-left: dock.shown(DockSide.left);
        out property <string> shown-right: dock.shown(DockSide.right);
        out property <string> shown-top: dock.shown(DockSide.top);
        out property <string> shown-bottom: dock.shown(DockSide.bottom);
        out property <length> left-body-y: dock.body-y(DockSide.left);
        out property <length> left-body-height: dock.body-height(DockSide.left);
        out property <length> left-width: dock.pane-width(DockSide.left);
        out property <length> bottom-y: dock.pane-y(DockSide.bottom);
        out property <length> center-x: dock.center-x;
        out property <length> center-width: dock.center-width;
        out property <length> center-height: dock.center-height;
        out property <bool> zones: dock.dragging;
        out property <int> floats: dock.floating.length;
        out property <length> float-x: dock.floating.length > 0 ? dock.floating[0].x : -1px;
        out property <int> panes: group.cells.length;
        out property <int> focused: focused-id();

        pure function focused-id() -> int {
            group.cells.length > 0 && group.cells[0].focused ? group.cells[0].id
            : group.cells.length > 1 && group.cells[1].focused ? group.cells[1].id
            : group.cells.length > 2 && group.cells[2].focused ? group.cells[2].id : -1
        }
        public pure function cell-x(index: int) -> length { group.cell-x(group.cells[index]) }
        public pure function cell-y(index: int) -> length { group.cell-y(group.cells[index]) }
        public pure function cell-width(index: int) -> length { group.cell-width(group.cells[index]) }
        public pure function cell-height(index: int) -> length { group.cell-height(group.cells[index]) }

        Rectangle {
            x: 20px;
            y: 10px;
            width: 760px;
            height: 380px;
            dock := Dock {
                name: "demo";
                for side in [DockSide.left, DockSide.right, DockSide.top, DockSide.bottom]: Rectangle {
                    x: dock.body-x(side);
                    y: dock.body-y(side);
                    width: dock.body-width(side);
                    height: dock.body-height(side);
                }
                for float in dock.floating: DockFloat {
                    dock: dock.name;
                    panel: float;
                    Text { text: float.id; }
                }
            }
        }

        Rectangle {
            x: 20px;
            y: 400px;
            width: 760px;
            height: 190px;
            group := PaneGroup {
                name: "editor";
            }
        }
    }
}

fn panel(id: &str, title: &str, icon: &str) -> DockPanel {
    DockPanel { id: id.into(), title: title.into(), icon: icon.into() }
}

// The gallery's demo: two panels on the left, one right, one at the bottom; two panes.
fn harness() -> (Harness, Rc<WorkspaceHost>) {
    let h = Harness::new().unwrap();
    let host = ely_slint::wire_workspace!(h);
    host.update(|systems| {
        let mut dock = DockModel::new();
        dock.add(panel("files", "Files", "folder"), Side::Left);
        dock.add(panel("outline", "Outline", "layers"), Side::Left);
        dock.add(panel("search", "Search", "search"), Side::Right);
        dock.add(panel("terminal", "Terminal", "terminal"), Side::Bottom);
        systems.docks.insert("demo".into(), dock);
        let mut group = PaneModel::new();
        group.split(1, Axis::Horizontal);
        systems.groups.insert("editor".into(), group);
    });
    show(&h);
    advance(16);
    (h, host)
}

fn center(h: &Harness, label: &str, nth: usize) -> (f32, f32) {
    let found: Vec<_> = ElementHandle::find_by_accessible_label(h, label).collect();
    let element = found.get(nth).unwrap_or_else(|| panic!("{label} #{nth} of {}", found.len()));
    let (at, size) = (element.absolute_position(), element.size());
    (at.x + size.width / 2.0, at.y + size.height / 2.0)
}

// Left and right 240 px with 6 px handles, the bottom 160 px; the center takes the rest. A side's
// body starts under its 32 px tab strip.
fn sides_frame_the_center() {
    let (h, _host) = harness();
    assert_eq!((h.get_shown_left().as_str(), h.get_shown_right().as_str()), ("files", "search"));
    assert_eq!((h.get_shown_top().as_str(), h.get_shown_bottom().as_str()), ("", "terminal"));
    assert_eq!(h.get_left_width(), 240.0);
    assert_eq!((h.get_center_x(), h.get_center_width()), (246.0, 760.0 - 2.0 * 246.0));
    assert_eq!(h.get_center_height(), 380.0 - 166.0);
    assert_eq!(h.get_bottom_y(), 380.0 - 160.0);
    assert_eq!((h.get_left_body_y(), h.get_left_body_height()), (32.0, 348.0));
}

fn a_tab_click_shows_its_panel() {
    let (h, _host) = harness();
    let (x, y) = center(&h, "Outline", 0);
    click(h.window(), x, y);
    advance(16);
    assert_eq!(h.get_shown_left().as_str(), "outline");
    assert!(!h.get_zones(), "a click is no drag");
}

// Past 2 px the tab drags and the zones show; let go over the right zone, it docks there.
fn a_tab_dropped_on_a_zone_docks_there() {
    let (h, host) = harness();
    let (x, y) = center(&h, "Outline", 0);
    mouse_down(h.window(), x, y);
    mouse_move(h.window(), x + 2.0, y);
    advance(16);
    assert!(!h.get_zones());
    mouse_move(h.window(), x + 3.0, y);
    advance(16);
    assert!(h.get_zones());
    // The right zone: 8 px in from the right, a fifth of the width.
    mouse_move(h.window(), 20.0 + 680.0, 10.0 + 200.0);
    mouse_up(h.window(), 20.0 + 680.0, 10.0 + 200.0);
    advance(16);
    assert!(!h.get_zones());
    assert_eq!((h.get_shown_left().as_str(), h.get_shown_right().as_str()), ("files", "outline"));
    let spots = host.read(|systems| systems.docks["demo"].layout().spots);
    assert_eq!(spots[1], ("outline".into(), Spot::Docked(Side::Right)));
}

// The float zone floats it at the pointer, in window coordinates.
fn a_tab_dropped_mid_dock_floats() {
    let (h, _host) = harness();
    let (x, y) = center(&h, "Search", 0);
    mouse_down(h.window(), x, y);
    mouse_move(h.window(), x - 10.0, y);
    mouse_move(h.window(), 400.0, 200.0);
    mouse_up(h.window(), 400.0, 200.0);
    advance(16);
    assert_eq!(h.get_floats(), 1);
    assert_eq!(h.get_float_x(), 400.0);
    assert_eq!(h.get_shown_right().as_str(), "", "the right side has nothing left");
    assert_eq!(h.get_center_width(), 760.0 - 246.0, "and is not drawn");
}

// Its title bar moves it; the model keeps the corner in window coordinates.
fn a_floating_panel_moves_by_its_bar() {
    let (h, host) = harness();
    host.update(|systems| systems.docks.get_mut("demo").unwrap().move_panel("search", Spot::Floating { x: 400.0, y: 200.0 }));
    advance(16);
    mouse_down(h.window(), 450.0, 214.0);
    mouse_move(h.window(), 455.0, 214.0);
    mouse_move(h.window(), 505.0, 234.0);
    mouse_up(h.window(), 505.0, 234.0);
    advance(16);
    let spots = host.read(|systems| systems.docks["demo"].layout().spots);
    assert_eq!(spots[2].1, Spot::Floating { x: 450.0, y: 220.0 });
}

// Let go off every zone: nothing moves.
fn a_tab_dropped_off_the_zones_stays() {
    let (h, _host) = harness();
    let (x, y) = center(&h, "Files", 0);
    mouse_down(h.window(), x, y);
    mouse_move(h.window(), x + 10.0, y);
    mouse_move(h.window(), 20.0 + 380.0, 10.0 + 4.0);
    mouse_up(h.window(), 20.0 + 380.0, 10.0 + 4.0);
    advance(16);
    assert_eq!(h.get_shown_left().as_str(), "files");
    assert_eq!(h.get_floats(), 0);
}

// The handle sets the span from the dock's edge, between 160 px and 45 % of the dock.
fn a_side_handle_resizes_within_bounds() {
    let (h, host) = harness();
    mouse_down(h.window(), 20.0 + 243.0, 100.0);
    mouse_move(h.window(), 20.0 + 250.0, 100.0);
    mouse_move(h.window(), 20.0 + 300.0, 100.0);
    advance(16);
    assert_eq!(h.get_left_width(), 300.0);
    assert_eq!(h.get_center_x(), 306.0);
    mouse_move(h.window(), 20.0 + 600.0, 100.0);
    advance(16);
    assert!((h.get_left_width() - 760.0 * 0.45).abs() < 0.01);
    mouse_move(h.window(), 20.0 + 10.0, 100.0);
    advance(16);
    assert_eq!(h.get_left_width(), 160.0);
    mouse_up(h.window(), 20.0 + 10.0, 100.0);
    assert_eq!(host.read(|systems| systems.docks["demo"].layout().sizes[0]), 10.0, "saved in rems");
}

// Two panes share 760 px less the 6 px handle; the second, split last, is focused.
fn panes_share_the_group() {
    let (h, _host) = harness();
    assert_eq!(h.get_panes(), 2);
    assert_eq!(h.get_focused(), 2);
    assert_eq!((h.invoke_cell_x(0), h.invoke_cell_width(0)), (0.0, 377.0));
    assert_eq!((h.invoke_cell_x(1), h.invoke_cell_width(1)), (383.0, 377.0));
    assert_eq!(h.invoke_cell_height(1), 190.0);
}

// The header's buttons split right or down and close; the new pane takes focus.
fn header_buttons_split_and_close() {
    let (h, host) = harness();
    let (x, y) = center(&h, "panel-bottom", 1);
    click(h.window(), x, y);
    advance(16);
    assert_eq!(h.get_panes(), 3);
    assert_eq!(h.get_focused(), 3);
    assert_eq!((h.invoke_cell_y(1), h.invoke_cell_height(1)), (0.0, 92.0));
    assert_eq!(h.invoke_cell_y(2), 98.0);
    // Closing pane 2 (its press focuses it first) hands focus to the first pane.
    let (x, y) = center(&h, "x", 1);
    click(h.window(), x, y);
    advance(16);
    assert_eq!(h.get_panes(), 2);
    assert_eq!(host.read(|systems| systems.groups["editor"].panes()), [1, 3]);
    assert_eq!(h.get_focused(), 1);
}

fn the_last_pane_cannot_close() {
    let (h, host) = harness();
    let (x, y) = center(&h, "x", 1);
    click(h.window(), x, y);
    advance(16);
    assert_eq!(h.get_panes(), 1);
    let (x, y) = center(&h, "x", 0);
    click(h.window(), x, y);
    advance(16);
    assert_eq!(host.read(|systems| systems.groups["editor"].panes()), [1]);
}

fn a_press_focuses_a_pane() {
    let (h, _host) = harness();
    mouse_down(h.window(), 20.0 + 100.0, 400.0 + 120.0);
    mouse_up(h.window(), 20.0 + 100.0, 400.0 + 120.0);
    advance(16);
    assert_eq!(h.get_focused(), 1);
}

// The divider drags from its first move, in shares of the split; no pane under 160 px.
fn a_divider_drag_resizes_the_panes() {
    let (h, _host) = harness();
    mouse_down(h.window(), 20.0 + 380.0, 500.0);
    mouse_move(h.window(), 20.0 + 390.0, 500.0);
    mouse_move(h.window(), 20.0 + 450.0, 500.0);
    advance(16);
    let share = 0.5 + 60.0 / 760.0;
    assert!((h.invoke_cell_width(0) - (754.0_f32 * share).round()).abs() <= 1.0, "{}", h.invoke_cell_width(0));
    mouse_move(h.window(), 20.0 - 400.0, 500.0);
    advance(16);
    assert!((h.invoke_cell_width(0) - (754.0_f32 * 160.0 / 760.0).round()).abs() <= 1.0, "{}", h.invoke_cell_width(0));
    mouse_up(h.window(), 20.0 - 400.0, 500.0);
}

// Save, change, restore: the view follows; a refused layout changes nothing.
fn a_saved_workspace_restores() {
    let (h, host) = harness();
    let saved = host.read(|systems| systems.capture("demo", "editor")).unwrap().to_json();
    host.update(|systems| {
        systems.docks.get_mut("demo").unwrap().move_panel("files", Spot::Floating { x: 40.0, y: 60.0 });
        systems.groups.get_mut("editor").unwrap().close(2);
    });
    advance(16);
    assert_eq!((h.get_floats(), h.get_panes()), (1, 1));
    let mut newer: serde_json::Value = serde_json::from_str(&saved).unwrap();
    newer["version"] = serde_json::json!(2);
    assert!(Workspace::from_json(&newer.to_string()).is_err());
    let layout = Workspace::from_json(&saved).unwrap();
    host.update(|systems| systems.apply(&layout, "demo", "editor")).unwrap();
    advance(16);
    assert_eq!((h.get_floats(), h.get_panes()), (0, 2));
    assert_eq!(h.get_shown_left().as_str(), "files");
}

cases!(dock_panes:
    sides_frame_the_center,
    a_tab_click_shows_its_panel,
    a_tab_dropped_on_a_zone_docks_there,
    a_tab_dropped_mid_dock_floats,
    a_floating_panel_moves_by_its_bar,
    a_tab_dropped_off_the_zones_stays,
    a_side_handle_resizes_within_bounds,
    panes_share_the_group,
    header_buttons_split_and_close,
    the_last_pane_cannot_close,
    a_press_focuses_a_pane,
    a_divider_drag_resizes_the_panes,
    a_saved_workspace_restores,
);
