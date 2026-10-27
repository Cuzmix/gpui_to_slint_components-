// Toolbar, StatusBar, ActivityBar, TabBar and the badges, after ely-core/src/shell/bars.rs,
// activity.rs, tabs.rs and data_display/badge.rs.
#[macro_use]
mod common;
use std::cell::RefCell;
use std::rc::Rc;

use common::*;
use i_slint_backend_testing::{AccessibleRole, ElementHandle};
use slint::{ComponentHandle, ModelRc, VecModel};

slint::slint! {
    import { Toolbar, ToolbarGroup, ToolbarSeparator, StatusBar, StatusBarSide, StatusBarItem, ActivityBar, ActivityItem, TabBar, WindowTab, CountBadge, IconButton, Icons } from "ui/index.slint";
    export { ActivityItem, WindowTab }
    export component Harness inherits Window {
        width: 700px;
        height: 500px;
        in property <int> count: 12;
        in property <string> chosen: "files";
        in property <[WindowTab]> tabs;
        in property <string> tab: "b";
        out property <int> status-clicks;
        out property <string> picked;
        out property <length> toolbar-h: tools.height;
        out property <length> sep-w: sep.width;
        out property <length> status-h: status.height;
        out property <length> badge-w: badge.width;
        out property <length> rail-w: rail.width;
        callback select-tab(string);
        callback close-tab(string);
        callback add-tab();
        callback reorder(int, int);

        VerticalLayout {
            alignment: start;
            spacing: 10px;
            tools := Toolbar {
                ToolbarGroup {
                    IconButton {
                        icon: Icons.bold;
                        icon-name: "bold";
                    }
                }
                sep := ToolbarSeparator { }
                ToolbarGroup {
                    IconButton {
                        icon: Icons.italic;
                        icon-name: "italic";
                    }
                }
            }

            status := StatusBar {
                StatusBarSide {
                    StatusBarItem {
                        label: "main";
                        clickable: true;
                        clicked => {
                            root.status-clicks += 1;
                        }
                    }
                    StatusBarItem {
                        label: "Synced";
                    }
                }
                StatusBarSide {
                    end: true;
                    StatusBarItem {
                        label: "UTF-8";
                    }
                }
            }

            HorizontalLayout {
                alignment: start;
                badge := CountBadge {
                    count: root.count;
                }
            }

            HorizontalLayout {
                alignment: start;
                height: 300px;
                rail := ActivityBar {
                    items: [
                        { id: "files", icon: Icons.folder, label: "Explorer" },
                        { id: "search", icon: Icons.search, label: "Search" },
                        { id: "git", icon: Icons.git-branch, label: "Source Control", badge: 3 },
                    ];
                    footer: [{ id: "settings", icon: Icons.settings, label: "Settings" }];
                    selected: root.chosen;
                    select(id) => {
                        root.picked = id;
                    }
                }

                TabBar {
                    width: 548px;
                    tabs: root.tabs;
                    selected: root.tab;
                    closable: true;
                    addable: true;
                    reorderable: true;
                    select(id) => {
                        root.select-tab(id);
                    }
                    close(id) => {
                        root.close-tab(id);
                    }
                    add => {
                        root.add-tab();
                    }
                    reorder(from, to) => {
                        root.reorder(from, to);
                    }
                }
            }
        }
    }
}

struct Box_ {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

fn find(h: &Harness, label: &str) -> Vec<Box_> {
    ElementHandle::find_by_accessible_label(h, label)
        .map(|e| {
            let (at, size) = (e.absolute_position(), e.size());
            Box_ { x: at.x, y: at.y, w: size.width, h: size.height }
        })
        .collect()
}

fn one(h: &Harness, label: &str) -> Box_ {
    let mut found = find(h, label);
    assert!(!found.is_empty(), "nothing labelled {label:?}");
    found.remove(0)
}

fn tab(id: &str, title: &str) -> WindowTab {
    WindowTab { id: id.into(), title: title.into(), icon: Default::default() }
}

#[derive(Default)]
struct Heard {
    selected: Vec<String>,
    closed: Vec<String>,
    added: u32,
    moved: Vec<(i32, i32)>,
}

fn harness() -> (Harness, Rc<VecModel<WindowTab>>, Rc<RefCell<Heard>>) {
    let h = Harness::new().unwrap();
    let tabs = Rc::new(VecModel::from(vec![tab("a", "Inbox"), tab("b", "Roadmap"), tab("c", "Meeting notes")]));
    h.set_tabs(ModelRc::from(tabs.clone()));
    let heard = Rc::new(RefCell::new(Heard::default()));
    let log = heard.clone();
    h.on_select_tab(move |id| log.borrow_mut().selected.push(id.into()));
    let log = heard.clone();
    h.on_close_tab(move |id| log.borrow_mut().closed.push(id.into()));
    let log = heard.clone();
    h.on_add_tab(move || log.borrow_mut().added += 1);
    let log = heard.clone();
    h.on_reorder(move |from, to| log.borrow_mut().moved.push((from, to)));
    show(&h);
    advance(16);
    (h, tabs, heard)
}

fn center(b: &Box_) -> (f32, f32) {
    (b.x + b.w / 2.0, b.y + b.h / 2.0)
}

// Toolbar 32 px; the separator keeps 4 px each side of its hairline. Status bar 24 px; a
// clickable item is a button, a plain one is not.
fn bars_keep_their_sizes_and_items_act() {
    let (h, _, _) = harness();
    assert_eq!(h.get_toolbar_h(), 32.0);
    assert_eq!(h.get_sep_w(), 9.0);
    assert_eq!(h.get_status_h(), 24.0);
    let main = find(&h, "main");
    // The button over the item carries the name too.
    assert_eq!(main.len(), find(&h, "Synced").len() + 1);
    let (x, y) = center(&main[0]);
    click(h.window(), x, y);
    assert_eq!(h.get_status_clicks(), 1);
}

// Zero hides a count badge; past 99 it reads 99+; 99 reads as is.
fn a_count_badge_caps_and_hides() {
    let (h, _, _) = harness();
    assert!(h.get_badge_w() > 0.0);
    h.set_count(0);
    advance(16);
    assert_eq!(h.get_badge_w(), 0.0);
    h.set_count(128);
    advance(16);
    assert!(!find(&h, "99+").is_empty());
    h.set_count(99);
    advance(800);
    assert!(find(&h, "99+").is_empty());
    assert!(!find(&h, "99").is_empty());
}

// The rail is 48 px; a press picks an item; the chosen one's mark grows in.
fn the_activity_bar_picks_and_marks() {
    let (h, _, _) = harness();
    assert_eq!(h.get_rail_w(), 48.0);
    let search = one(&h, "Search");
    let (x, y) = center(&search);
    click(h.window(), x, y);
    assert_eq!(h.get_picked(), "search");
    assert_eq!((search.w, search.h), (48.0, 48.0));
    // The badge on Source Control reads 3.
    assert!(!find(&h, "3").is_empty());
    // Settings sits at the foot.
    let settings = one(&h, "Settings");
    assert!(settings.y > search.y + 100.0);
}

// The chosen item's mark grows from nothing to the icon's height over BASE.
fn the_activity_mark_grows_in() {
    let (h, _, _) = harness();
    let mark = || ElementHandle::find_by_element_id(&h, "ActivityEntry::mark").next().expect("a mark").size().height;
    // 16 ms in: 1 − (1 − 16/200)³ of 20 px.
    assert!((mark() - 20.0 * (1.0 - 0.92f32.powi(3))).abs() < 0.05, "16 ms in: {}", mark());
    advance(234);
    assert_eq!(mark(), 20.0);
}

// A new tab's title rises 4 px into place over BASE.
fn a_new_tab_rises_in() {
    let (h, tabs, _) = harness();
    let title_y = |label: &str| {
        ElementHandle::find_by_accessible_label(&h, label)
            .find(|e| e.accessible_role() == Some(AccessibleRole::Text))
            .expect("a title")
            .absolute_position()
            .y
    };
    tabs.push(tab("f", "Fresh"));
    advance(0);
    let start = title_y("Fresh");
    advance(100);
    let mid = title_y("Fresh");
    advance(150);
    let rest = title_y("Fresh");
    assert!((start - rest - 4.0).abs() < 0.01, "starts 4 px low: {}", start - rest);
    assert!(mid > rest && mid < start - 2.0, "eases out: {}", mid - rest);
}

// A press picks a tab; the chosen tab's close closes it; the plus adds one.
fn tabs_pick_close_and_add() {
    let (h, _, heard) = harness();
    let (x, y) = center(&one(&h, "Inbox"));
    click(h.window(), x, y);
    assert_eq!(heard.borrow().selected, vec!["a".to_string()]);
    let (x, y) = center(&one(&h, "Close Roadmap"));
    click(h.window(), x, y);
    assert_eq!(heard.borrow().closed, vec!["b".to_string()]);
    assert_eq!(heard.borrow().selected.len(), 1, "the close does not select");
    let (x, y) = center(&one(&h, "New tab"));
    click(h.window(), x, y);
    assert_eq!(heard.borrow().added, 1);
}

// Tabs share the width evenly between 96 and 200 px.
fn tabs_share_the_width_within_their_limits() {
    let (h, tabs, _) = harness();
    // 548 px less two gaps and the 24 px button with its gap, in three.
    let a = one(&h, "Inbox");
    assert!((a.w - (548.0 - 8.0 - 28.0) / 3.0).abs() < 0.01, "{}", a.w);
    for ix in 0..5 {
        tabs.push(tab(&format!("d{ix}"), &format!("Draft {ix}")));
    }
    advance(16);
    let a = one(&h, "Inbox");
    let b = one(&h, "Roadmap");
    assert_eq!(a.w, b.w);
    assert_eq!(a.w, 96.0, "eight tabs stop at the least width");
}

// A tab dragged onto another reports (from, to); onto itself, nothing.
fn a_dragged_tab_reorders_on_drop() {
    let (h, _, heard) = harness();
    let (x, y) = center(&one(&h, "Inbox"));
    let (tx, _) = center(&one(&h, "Meeting notes"));
    mouse_move(h.window(), x, y);
    mouse_down(h.window(), x, y);
    mouse_move(h.window(), x + 10.0, y);
    mouse_move(h.window(), tx, y);
    mouse_up(h.window(), tx, y);
    advance(16);
    assert_eq!(heard.borrow().moved, vec![(0, 2)]);
    assert!(heard.borrow().selected.is_empty(), "a drag does not select");
    mouse_move(h.window(), x, y);
    mouse_down(h.window(), x, y);
    mouse_move(h.window(), x + 10.0, y);
    mouse_up(h.window(), x + 10.0, y);
    advance(16);
    assert_eq!(heard.borrow().moved.len(), 1);
}

cases!(bars: the_activity_mark_grows_in, a_new_tab_rises_in, bars_keep_their_sizes_and_items_act, a_count_badge_caps_and_hides, the_activity_bar_picks_and_marks, tabs_pick_close_and_add, tabs_share_the_width_within_their_limits, a_dragged_tab_reorders_on_drop);
