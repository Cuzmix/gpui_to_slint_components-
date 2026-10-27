#[macro_use]
mod common;
use common::*;

slint::slint! {
    import { Card, CardHeader, CardBody, CardFooter, Panel, PanelHeader, Section, SectionHeader, Fieldset, Frame, Well, Theme } from "ui/index.slint";
    export component Harness inherits Window {
        width: 1200px;
        height: 900px;
        out property <length> head-x: action.absolute-position.x;
        out property <length> head-y: action.absolute-position.y;
        out property <length> body-x: body-item.absolute-position.x;
        out property <length> body-y: body-item.absolute-position.y;
        out property <length> body2-y: body-item2.absolute-position.y;
        out property <length> card-h: card.height;
        out property <length> foot-x: foot-item.absolute-position.x;
        out property <length> foot-y: foot-item.absolute-position.y;
        out property <length> panel-body-y: panel-body.absolute-position.y;
        out property <length> panel-body-h: panel-body.height;
        out property <length> panel-act-x: panel-act.absolute-position.x;
        out property <length> panel-act-y: panel-act.absolute-position.y;
        out property <length> sec-act-y: sec-act.absolute-position.y;
        out property <length> sec-act-x: sec-act.absolute-position.x;
        out property <length> sec-body-y: sec-body.absolute-position.y;
        out property <length> sec-head-h: sec-head.height;
        out property <length> field-x: field-item.absolute-position.x;
        out property <length> field-y: field-item.absolute-position.y;
        out property <length> frame-x: frame-item.absolute-position.x;
        out property <length> well-y: well-item.absolute-position.y;
        out property <bool> well-fill: well.background == Theme.colors.sunken;
        out property <bool> frame-line: frame.border-color == Theme.colors.border-strong;

        VerticalLayout {
            x: 0;
            y: 0;
            width: 380px;
            alignment: start;
            card := Card {
                CardHeader {
                    title: "Storage";
                    description: "Across all workspaces";
                    action := Rectangle { width: 24px; height: 24px; }
                }
                CardBody {
                    body-item := Rectangle { height: 20px; }
                    body-item2 := Rectangle { height: 20px; }
                }
                CardFooter {
                    foot-item := Rectangle { width: 60px; height: 28px; }
                }
            }
        }

        Panel {
            x: 400px;
            y: 0;
            width: 320px;
            height: 160px;
            PanelHeader {
                title: "Outline";
                Rectangle { width: 24px; height: 24px; }
                panel-act := Rectangle { width: 24px; height: 24px; }
            }
            panel-body := Rectangle { }
        }

        VerticalLayout {
            x: 0;
            y: 400px;
            width: 520px;
            alignment: start;
            Section {
                sec-head := SectionHeader {
                    title: "Members";
                    description: "People who can open this workspace.";
                    sec-act := Rectangle { width: 70px; height: 28px; }
                }
                sec-body := Rectangle { height: 30px; }
            }
        }

        VerticalLayout {
            x: 600px;
            y: 400px;
            width: 420px;
            alignment: start;
            Fieldset {
                legend: "Shipping address";
                field-item := Rectangle { height: 30px; }
            }
            frame := Frame {
                frame-item := Rectangle { height: 30px; }
            }
            well := Well {
                well-item := Rectangle { height: 30px; }
            }
        }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    show(&h);
    advance(16);
    h
}

// Every part sits inside the 1 px hairline.
fn a_card_lays_out_its_parts() {
    let h = harness();
    // The action: top right of the header, 20 px in.
    assert_eq!((h.get_head_x(), h.get_head_y()), (380.0 - 1.0 - 20.0 - 24.0, 21.0));
    // Header: title md (14 px, phi: 23) + 2 + description sm (12 px: 19) = 44, from 21.
    assert_eq!(h.get_body_x(), 21.0);
    assert_eq!(h.get_body_y(), 21.0 + 44.0 + 20.0);
    assert_eq!(h.get_body2_y(), h.get_body_y() + 32.0);
    // Footer: a hairline, then 12 px; its child at the end, 20 px in.
    let footer_top = h.get_body2_y() + 20.0 + 20.0;
    assert_eq!(h.get_foot_y(), footer_top + 13.0);
    assert_eq!(h.get_foot_x(), 380.0 - 1.0 - 20.0 - 60.0);
    assert_eq!(h.get_card_h(), footer_top + 13.0 + 28.0 + 12.0 + 1.0);
}

fn a_panel_has_a_bar_then_its_body() {
    let h = harness();
    assert_eq!(h.get_panel_body_y(), 1.0 + 32.0);
    assert_eq!(h.get_panel_body_h(), 160.0 - 2.0 - 32.0);
    assert_eq!(h.get_panel_act_x(), 400.0 + 320.0 - 1.0 - 4.0 - 24.0);
    // Centered in the 31 px above the bar's hairline.
    assert_eq!(h.get_panel_act_y(), 1.0 + 3.5);
}

fn a_section_heads_its_body() {
    let h = harness();
    // Title lg (16 px: 26) + 2 + sm line (19) = 47; the action sits on the bottom.
    assert_eq!(h.get_sec_head_h(), 47.0);
    assert_eq!(h.get_sec_act_y(), 400.0 + 47.0 - 28.0);
    assert_eq!(h.get_sec_act_x(), 520.0 - 70.0);
    assert_eq!(h.get_sec_body_y(), 400.0 + 47.0 + 16.0);
}

fn fieldset_frame_and_well_pad_inside_their_line() {
    let h = harness();
    // Legend xs (11 px: 18) + 8, then 1 + 16.
    assert_eq!((h.get_field_x(), h.get_field_y()), (600.0 + 17.0, 400.0 + 18.0 + 8.0 + 17.0));
    assert_eq!(h.get_frame_x(), 600.0 + 17.0);
    assert!(h.get_frame_line());
    assert!(h.get_well_fill());
    let frame_top = 400.0 + 18.0 + 8.0 + 64.0;
    assert_eq!(h.get_well_y(), frame_top + 64.0 + 17.0);
}

cases!(surface: a_card_lays_out_its_parts, a_panel_has_a_bar_then_its_body, a_section_heads_its_body, fieldset_frame_and_well_pad_inside_their_line);
