// At rest nothing keeps redrawing: every tick-driven glide, trip and entrance stops reading
// `animation-tick()` once it is over (gpui animations end with their duration).
#[macro_use]
mod common;
use common::*;
use slint::ComponentHandle;

slint::slint! {
    import { ProgressBar, ProgressRing, SegmentedControl, OfflineIndicator, Connectivity, ActionBar, BulkActionBar, FloatingActionButton, ConfirmButton, TabBar, AnimatedNumber, Collapsible, Icons, TextFormat, Button } from "ui/index.slint";
    export { Connectivity, TextFormat }
    export component Harness inherits Window {
        width: 600px;
        height: 600px;
        in-out property <float> progress: 0.2;
        in-out property <string> view: "a";
        in-out property <Connectivity> link: Connectivity.offline;
        in-out property <int> picked;
        in-out property <bool> open;
        VerticalLayout {
            alignment: start;
            spacing: 8px;
            ProgressBar { value: root.progress; }
            ProgressRing { value: root.progress; }
            HorizontalLayout {
                alignment: start;
                SegmentedControl {
                    segments: [{ value: "a", label: "Alpha" }, { value: "b", label: "Beta" }];
                    selected <=> root.view;
                }
            }
            OfflineIndicator { state: root.link; }
            ActionBar { Button { text: "Save"; } }
            HorizontalLayout {
                alignment: start;
                BulkActionBar { count: root.picked; }
            }
            HorizontalLayout {
                alignment: start;
                FloatingActionButton { icon: Icons.plus; }
                ConfirmButton { text: "Delete"; }
            }
            AnimatedNumber { value: root.picked; }
            Collapsible {
                open: root.open;
                Rectangle { height: 40px; }
            }
        }
    }
}

fn settle(h: &Harness) {
    for _ in 0..200 {
        advance(16);
    }
    assert!(!h.window().has_active_animations(), "still animating at rest");
}

fn nothing_ticks_at_rest() {
    let h = Harness::new().unwrap();
    ely_slint::wire_text_format!(h);
    show(&h);
    settle(&h);
    h.set_progress(0.7);
    h.set_view("b".into());
    h.set_link(Connectivity::Online);
    h.set_picked(4);
    h.set_open(true);
    advance(16);
    assert!(h.window().has_active_animations(), "the changes animate");
    settle(&h);
}

cases!(idle: nothing_ticks_at_rest);
