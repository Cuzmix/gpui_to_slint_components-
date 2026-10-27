#[macro_use]
mod common;
use common::*;
use i_slint_backend_testing::ElementHandle;

slint::slint! {
    import { Sidebar, Sheet, Drawer, Place, Page, PageHeader, PageContent, AppShell, AppShellBar, AppShellMain, AppShellContent, MasterDetail, SplitCell, ResizeHandle, LayoutMath, MasonrySpot, Theme } from "ui/index.slint";
    export { LayoutMath, MasonrySpot }
    export component Harness inherits Window {
        width: 1000px;
        height: 700px;
        in-out property <bool> collapsed: false;
        out property <length> side-w: side.width;
        in-out property <bool> sheet-open: false;
        in-out property <Place> side-of: Place.right;
        out property <int> sheet-closes;
        in-out property <bool> drawer-open: false;
        out property <int> drawer-closes;
        out property <length> act-y: act.absolute-position.y;
        out property <length> act-x: act.absolute-position.x;
        out property <length> content-y: content.absolute-position.y;
        out property <length> shell-main-y: main-item.absolute-position.y;
        out property <length> shell-main-h: main-item.height;
        out property <length> shell-content-x: main-item.absolute-position.x;
        out property <length> status-y: status.absolute-position.y;
        out property <length> master-w: master.width;

        Rectangle {
            x: 0;
            y: 0;
            width: 560px;
            height: 200px;
            HorizontalLayout {
                side := Sidebar { collapsed: root.collapsed; }
                Rectangle { }
            }
        }

        Rectangle {
            x: 0;
            y: 220px;
            width: 640px;
            height: 240px;
            Page {
                PageHeader {
                    title: "Projects";
                    subtitle: "Everything your team is shipping.";
                    act := Rectangle { width: 100px; height: 28px; }
                }
                PageContent {
                    content := Rectangle { height: 30px; }
                }
            }
        }

        Rectangle {
            x: 660px;
            y: 220px;
            width: 300px;
            height: 200px;
            AppShell {
                AppShellBar { Rectangle { height: 28px; } }
                AppShellMain {
                    Rectangle { width: 56px; }
                    AppShellContent {
                        main-item := Rectangle { }
                    }
                }
                AppShellBar { status := Rectangle { height: 28px; } }
            }
        }

        Rectangle {
            x: 0;
            y: 480px;
            width: 606px;
            height: 100px;
            md := MasterDetail {
                min: 240px;
                master := SplitCell { share: md.share(0); }
                ResizeHandle { orientation: md.orientation; grabbed => { md.grab(0); } moved(a) => { md.drag(0, a); } }
                SplitCell { share: md.share(1); }
            }
        }

        if root.sheet-open: Sheet {
            side: root.side-of;
            title: "Filters";
            close => {
                root.sheet-closes += 1;
                root.sheet-open = false;
            }
            sheet-body := Rectangle { height: 40px; }
        }

        if root.drawer-open: Drawer {
            close => {
                root.drawer-closes += 1;
                root.drawer-open = false;
            }
            Rectangle { height: 100px; }
        }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    ely_slint::wire_layout!(h);
    show(&h);
    advance(16);
    h
}

fn ease_in_out_cubic(t: f32) -> f32 {
    if t < 0.5 { 4.0 * t * t * t } else { 1.0 - (-2.0 * t + 2.0).powi(3) / 2.0 }
}

fn a_sidebar_folds_to_a_rail() {
    let h = harness();
    assert_eq!(h.get_side_w(), 240.0);
    h.set_collapsed(true);
    advance(0);
    advance(100);
    let expected = 240.0 + (56.0 - 240.0) * ease_in_out_cubic(0.5);
    assert!((h.get_side_w() - expected).abs() < 0.5, "{}", h.get_side_w());
    advance(120);
    assert_eq!(h.get_side_w(), 56.0);
}

// The header's actions sit on its bottom, 40 px in; the body 32 px under the hairline.
fn a_page_heads_its_content() {
    let h = harness();
    // Title xxl (24 px at phi: 39) + 4 + md (14 px: 23) = 66, then 24 + 1.
    assert_eq!(h.get_act_x(), 640.0 - 40.0 - 100.0);
    assert_eq!(h.get_act_y(), 220.0 + 32.0 + 66.0 - 28.0);
    assert_eq!(h.get_content_y(), 220.0 + 32.0 + 66.0 + 25.0 + 32.0);
}

fn an_app_shell_frames_its_main_row() {
    let h = harness();
    assert_eq!(h.get_shell_main_y(), 220.0 + 28.0);
    assert_eq!(h.get_shell_main_h(), 200.0 - 56.0);
    assert_eq!(h.get_shell_content_x(), 660.0 + 56.0);
    assert_eq!(h.get_status_y(), 220.0 + 200.0 - 28.0);
}

fn master_detail_splits_36_64() {
    let h = harness();
    assert!((h.get_master_w() - 0.36 * 600.0).abs() <= 1.0, "{}", h.get_master_w());
}

// 380 px from the right; it slides in over 200 ms.
fn a_sheet_slides_in_and_closes() {
    let h = harness();
    h.set_sheet_open(true);
    advance(0);
    let close = || ElementHandle::find_by_accessible_label(&h, "x").next().unwrap();
    let start = close().absolute_position().x;
    advance(250);
    let settled = close().absolute_position().x;
    assert!(start > settled + 300.0, "{start} → {settled}");
    // 24 px in from the panel's right edge at 1000 px.
    assert_eq!(settled + close().size().width, 1000.0 - 24.0);
    let at = close().absolute_position();
    click(h.window(), at.x + 5.0, at.y + 5.0);
    assert_eq!(h.get_sheet_closes(), 1);
    h.set_sheet_open(true);
    advance(250);
    click(h.window(), 100.0, 100.0);
    assert_eq!(h.get_sheet_closes(), 2, "the scrim closes it");
}

// Body 100 + grab 24 + 16 gap + 32 bottom = 172 px; a pull past 30 % (51.6 px) dismisses.
fn a_drawer_pull_dismisses_or_springs_back() {
    let h = harness();
    h.set_drawer_open(true);
    advance(0);
    advance(250);
    let grab_y = 700.0 - 172.0 + 10.0;
    mouse_down(h.window(), 500.0, grab_y);
    mouse_move(h.window(), 500.0, grab_y + 5.0);
    mouse_move(h.window(), 500.0, grab_y + 45.0);
    mouse_up(h.window(), 500.0, grab_y + 45.0);
    assert_eq!(h.get_drawer_closes(), 0, "40 px is short of 51.6");
    advance(250);
    mouse_down(h.window(), 500.0, grab_y);
    mouse_move(h.window(), 500.0, grab_y + 5.0);
    mouse_move(h.window(), 500.0, grab_y + 65.0);
    mouse_up(h.window(), 500.0, grab_y + 65.0);
    assert_eq!(h.get_drawer_closes(), 1);
}

cases!(shells: a_sidebar_folds_to_a_rail, a_page_heads_its_content, an_app_shell_frames_its_main_row, master_detail_splits_36_64, a_sheet_slides_in_and_closes, a_drawer_pull_dismisses_or_springs_back);
