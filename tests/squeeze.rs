// Regressions from the layout page: wrapping text inside a split cell, and a masonry squeezed by
// a Flickable that hands its content only its minimum height.
#[macro_use]
mod common;
use common::*;

slint::slint! {
    import { SplitPane, SplitCell, ResizeHandle, Masonry, Paragraph, LayoutMath, MasonrySpot } from "ui/index.slint";
    export { LayoutMath, MasonrySpot }
    export component Harness inherits Window {
        width: 800px;
        height: 800px;
        out property <length> para-h: para.height;
        out property <length> para-w: para.width;
        out property <length> m-h: m.height;

        VerticalLayout {
            Rectangle {
                height: 240px;
                split := SplitPane {
                    count: 2;
                    SplitCell {
                        share: split.share(0);
                        VerticalLayout {
                            padding: 16px;
                            alignment: start;
                            para := Paragraph {
                                text: "Numbers, notes and the next three moves, written long enough to wrap onto more than one line in a narrow cell.";
                            }
                        }
                    }
                    ResizeHandle {
                        orientation: split.orientation;
                    }
                    SplitCell { share: split.share(1); }
                }
            }

            Flickable {
                height: 300px;
                // A sibling that may shrink, so the squeeze has somewhere to go.
                VerticalLayout {
                    m := Masonry {
                        columns: 2;
                        gap: 10px;
                        heights: [72px, 120px, 56px];
                        for h[i] in self.heights: Rectangle {
                            x: m.item-x(i);
                            y: m.item-y(i);
                            width: m.column-width;
                            height: h;
                        }
                    }
                    Rectangle {
                        preferred-height: 600px;
                    }
                }
            }
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

// Laid out without recursing; the text wraps to more than one line inside the cell.
fn wrapping_text_in_a_split_cell_lays_out() {
    let h = harness();
    assert!(h.get_para_w() > 0.0 && h.get_para_w() < 400.0);
    assert!(h.get_para_h() > 30.0);
}

// Columns 72+56+10 and 120: the masonry keeps its full height under a Flickable.
fn masonry_keeps_its_height_when_squeezed() {
    let h = harness();
    assert_eq!(h.get_m_h(), 138.0);
}

cases!(squeeze: wrapping_text_in_a_split_cell_lays_out, masonry_keeps_its_height_when_squeezed);
