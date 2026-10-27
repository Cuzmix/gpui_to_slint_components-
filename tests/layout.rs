#[macro_use]
mod common;

slint::slint! {
    import { HStack, VStack, ZStack, ZLayer, AspectRatio, Container, ContainerSize, SimpleGrid, Masonry, LayoutMath, MasonrySpot } from "ui/index.slint";
    export { LayoutMath, MasonrySpot }
    export component Harness inherits Window {
        width: 1400px;
        height: 1400px;
        out property <length> hs-w: hs.width;
        out property <length> hs-h: hs.height;
        out property <length> hs-b-y: hb.y;
        out property <length> hs-c-x: hc.x;
        out property <length> vs-w: vs.width;
        out property <length> vs-b-y: vb.y;
        out property <length> vs-b-w: vb.width;
        out property <length> z-w: z.width;
        out property <length> z-h: z.height;
        out property <length> z-layer-w: zl.width;
        out property <length> z-layer-h: zl.height;
        in-out property <float> ratio: 16 / 9;
        out property <length> ar-h: ar.height;
        out property <length> ar-w: ar.width;
        in-out property <length> host-w: 1000px;
        in-out property <ContainerSize> size: ContainerSize.sm;
        out property <length> box-x: box.absolute-position.x;
        out property <length> box-w: box.width;
        in-out property <length> grid-w: 720px;
        out property <int> grid-cols: grid.columns;
        out property <length> grid-cell: grid.cell-width;
        out property <length> item7-x: item7.x;
        out property <length> item7-y: item7.y;
        out property <length> grid-h: grid.height;
        out property <length> m-h: m.preferred-height;
        out property <length> m4-x: m.item-x(4);
        out property <length> m4-y: m.item-y(4);
        out property <length> m3-x: m.item-x(3);
        out property <length> m3-y: m.item-y(3);
        out property <length> m-col: m.column-width;

        VerticalLayout {
            x: 0;
            y: 0;
            width: 300px;
            alignment: start;
            HorizontalLayout {
                alignment: start;
                hs := HStack {
                    spacing: 8px;
                    Rectangle { width: 40px; height: 20px; }
                    hb := Rectangle { width: 40px; height: 40px; }
                    hc := Rectangle { width: 40px; height: 10px; }
                }
            }
            vs := VStack {
                spacing: 8px;
                Rectangle { height: 20px; }
                vb := Rectangle { height: 30px; }
            }
            HorizontalLayout {
                alignment: start;
                z := ZStack {
                    Rectangle { width: 200px; height: 120px; }
                    zl := ZLayer { Rectangle { preferred-width: 500px; preferred-height: 500px; } }
                }
            }
            HorizontalLayout {
                alignment: start;
                VerticalLayout {
                    width: 240px;
                    alignment: start;
                    ar := AspectRatio { ratio: root.ratio; }
                }
            }
        }

        VerticalLayout {
            x: 0;
            y: 400px;
            width: root.host-w;
            alignment: start;
            Container {
                size: root.size;
                box := Rectangle { height: 40px; }
            }
        }

        VerticalLayout {
            x: 0;
            y: 500px;
            width: root.grid-w;
            alignment: start;
            grid := SimpleGrid {
                min: 140px;
                gap: 12px;
                for n in 6: Rectangle { width: grid.cell-width; height: 56px; }
                item7 := Rectangle { width: grid.cell-width; height: 56px; }
            }
        }

        VerticalLayout {
            x: 0;
            y: 800px;
            width: 560px;
            alignment: start;
            m := Masonry {
                columns: 3;
                gap: 10px;
                heights: [72px, 120px, 56px, 96px, 140px];
                for h[i] in self.heights: Rectangle {
                    x: m.item-x(i);
                    y: m.item-y(i);
                    width: m.column-width;
                    height: h;
                }
            }
        }
    }
}

use common::*;

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    ely_slint::wire_layout!(h);
    show(&h);
    advance(16);
    h
}

// h_stack: a row at its content's size, items centered across it.
fn an_h_stack_centers_its_row() {
    let h = harness();
    assert_eq!(h.get_hs_w(), 136.0);
    assert_eq!(h.get_hs_h(), 40.0);
    assert_eq!(h.get_hs_b_y(), 0.0);
    assert_eq!(h.get_hs_c_x(), 96.0);
}

// v_stack: a column whose items stretch across it.
fn a_v_stack_stretches_its_items() {
    let h = harness();
    assert_eq!(h.get_vs_w(), 300.0);
    assert_eq!(h.get_vs_b_y(), 28.0);
    assert_eq!(h.get_vs_b_w(), 300.0);
}

fn a_z_stack_takes_the_ground_size() {
    let h = harness();
    assert_eq!((h.get_z_w(), h.get_z_h()), (200.0, 120.0));
    assert_eq!((h.get_z_layer_w(), h.get_z_layer_h()), (200.0, 120.0));
}

fn an_aspect_ratio_follows_the_width() {
    let h = harness();
    assert_eq!(h.get_ar_w(), 240.0);
    assert_eq!(h.get_ar_h(), 135.0);
    h.set_ratio(1.0);
    assert_eq!(h.get_ar_h(), 240.0);
    h.set_ratio(0.0);
    assert_eq!(h.get_ar_h(), 0.0);
}

// Capped at the token, centered, padded 24 px inside the cap.
fn a_container_caps_and_centers() {
    let h = harness();
    assert_eq!(h.get_box_w(), 640.0 - 48.0);
    assert_eq!(h.get_box_x(), 180.0 + 24.0);
    h.set_size(ContainerSize::Lg);
    assert_eq!(h.get_box_w(), 1000.0 - 48.0);
    assert_eq!(h.get_box_x(), 24.0);
}

// 720 px at 140 px with 12 px gaps: four columns; the seventh item opens row two, third place.
fn a_simple_grid_fits_equal_columns() {
    let h = harness();
    assert_eq!(h.get_grid_cols(), 4);
    assert_eq!(h.get_grid_cell(), (720.0 - 36.0) / 4.0);
    advance(16);
    assert_eq!(h.get_item7_x(), 2.0 * (171.0 + 12.0));
    assert_eq!(h.get_item7_y(), 56.0 + 12.0);
    assert_eq!(h.get_grid_h(), 56.0 * 2.0 + 12.0);
    h.set_grid_w(100.0);
    assert_eq!(h.get_grid_cols(), 1);
    assert_eq!(h.get_grid_cell(), 100.0);
}

fn masonry_joins_the_shortest_column() {
    let h = harness();
    let column = (560.0 - 20.0) / 3.0;
    assert_eq!(h.get_m_col(), column);
    assert_eq!((h.get_m3_x(), h.get_m3_y()), (2.0 * (column + 10.0), 66.0));
    assert_eq!((h.get_m4_x(), h.get_m4_y()), (0.0, 82.0));
    assert_eq!(h.get_m_h(), 222.0);
}

cases!(layout: an_h_stack_centers_its_row, a_v_stack_stretches_its_items, a_z_stack_takes_the_ground_size, an_aspect_ratio_follows_the_width, a_container_caps_and_centers, a_simple_grid_fits_equal_columns, masonry_joins_the_shortest_column);
