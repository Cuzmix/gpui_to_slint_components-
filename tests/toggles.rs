// ButtonGroup, ToggleButton and ToggleGroup, after ely-core/src/buttons/group.rs.
#[macro_use]
mod common;
use common::*;
use i_slint_backend_testing::{AccessibleRole, ElementHandle};
use slint::platform::Key;
use slint::{ComponentHandle, Model, SharedString};

slint::slint! {
    import { ButtonGroup, Button, GroupSlot, ToggleButton, ToggleGroup, ToggleItem, Toggles, Icons, Theme, TextSize } from "ui/index.slint";
    export { Toggles }
    export component Harness inherits Window {
        width: 600px;
        height: 400px;
        in-out property <bool> bold-on;
        in-out property <bool> wrap-enabled: true;
        in-out property <[string]> picked: ["left"];
        in-out property <bool> many;
        in-out property <length> group-width: 400px;
        in-out property <[string]> reported;
        in-out property <int> reports;
        in-out property <int> toggles;
        out property <bool> compact: icons.compact;
        out property <length> center-width: center-probe.preferred-width;
        out property <color> c-active: Theme.colors.active;
        out property <color> c-hover: Theme.colors.hover;
        out property <color> bold-fill: bold.current-fill;
        out property <bool> bold-focused: bold.has-focus;

        VerticalLayout {
            alignment: start;
            spacing: 8px;
            HorizontalLayout {
                alignment: start;
                ButtonGroup {
                    Button { text: "Previous"; group-slot: GroupSlot.first; }
                    Button { text: "Today"; group-slot: GroupSlot.middle; }
                    Button { text: "Next"; group-slot: GroupSlot.last; }
                }
            }
            HorizontalLayout {
                alignment: start;
                spacing: 8px;
                bold := ToggleButton {
                    item: { value: "bold", icon: Icons.bold, tooltip: "Bold" };
                    on <=> root.bold-on;
                    toggled(on) => { root.toggles += 1; }
                }
                ToggleButton {
                    item: { value: "wrap", label: "Wrap", icon: Icons.italic };
                    enabled: root.wrap-enabled;
                    toggled(on) => { root.toggles += 1; }
                }
            }
            HorizontalLayout {
                alignment: start;
                icons := ToggleGroup {
                    width: root.group-width;
                    items: [
                        { value: "left", label: "Left", icon: Icons.align-left },
                        { value: "center", label: "Center", icon: Icons.align-center },
                        { value: "right", label: "Right", icon: Icons.align-right },
                    ];
                    selected <=> root.picked;
                    multiple: root.many;
                    selection-changed(next) => {
                        root.reported = next;
                        root.reports += 1;
                    }
                }
            }
            HorizontalLayout {
                alignment: start;
                ToggleGroup {
                    width: 120px;
                    items: [
                        { value: "day", label: "Day", icon: Icons.sun },
                        { value: "plain", label: "Plain" },
                    ];
                }
            }
        }
        center-probe := Text {
            visible: false;
            text: "Center";
            font-size: Theme.text-size(TextSize.base);
            font-weight: 500;
            font-family: Theme.font-family;
        }
    }
}

struct Box_ {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

fn one(h: &Harness, label: &str) -> ElementHandle {
    let found: Vec<_> = ElementHandle::find_by_accessible_label(h, label)
        .filter(|e| e.accessible_role() == Some(AccessibleRole::Button))
        .collect();
    assert_eq!(found.len(), 1, "one button named {label}");
    found.into_iter().next().unwrap()
}

fn bounds(e: &ElementHandle) -> Box_ {
    let (at, size) = (e.absolute_position(), e.size());
    Box_ { x: at.x, y: at.y, w: size.width, h: size.height }
}

fn press(h: &Harness, label: &str) {
    let b = bounds(&one(h, label));
    click(h.window(), b.x + b.w / 2.0, b.y + b.h / 2.0);
}

fn strings(model: slint::ModelRc<SharedString>) -> Vec<String> {
    model.iter().map(|s| s.to_string()).collect()
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    ely_slint::wire_toggles!(h);
    show(&h);
    advance(16);
    h
}

// The group's buttons touch: each starts where the last ends, all one height.
fn a_button_group_joins_its_buttons() {
    let h = harness();
    let (a, b, c) = (bounds(&one(&h, "Previous")), bounds(&one(&h, "Today")), bounds(&one(&h, "Next")));
    assert_eq!(b.x, a.x + a.w);
    assert_eq!(c.x, b.x + b.w);
    assert_eq!((a.h, b.h, c.h), (28.0, 28.0, 28.0));
}

// A press flips it and reports the new state; on shows the active fill, off hovers. An icon
// alone is a square; the accessible name falls back to the tooltip, and it reads as checked.
fn a_toggle_button_flips_on_each_press() {
    let h = harness();
    let bold = one(&h, "Bold");
    let b = bounds(&bold);
    assert_eq!((b.w, b.h), (28.0, 28.0));
    assert_eq!(bold.accessible_role(), Some(AccessibleRole::Button));
    assert_eq!(bold.accessible_checked(), Some(false));
    mouse_move(h.window(), b.x + 4.0, b.y + 4.0);
    assert_eq!(h.get_bold_fill(), h.get_c_hover());
    press(&h, "Bold");
    assert!(h.get_bold_on());
    assert_eq!(h.get_toggles(), 1);
    assert_eq!(h.get_bold_fill(), h.get_c_active());
    assert_eq!(one(&h, "Bold").accessible_checked(), Some(true));
    assert!(!h.get_bold_focused(), "a press takes no focus");
    press(&h, "Bold");
    assert!(!h.get_bold_on());
    assert_eq!(h.get_toggles(), 2);
}

// Tab reaches it and Enter or Space toggles on release; disabled, it is no stop, ignores presses
// and reads as unavailable.
fn keys_toggle_and_disabled_ignores() {
    let h = harness();
    tab(h.window());
    tab(h.window());
    tab(h.window());
    tab(h.window());
    assert!(h.get_bold_focused());
    key_down(h.window(), Key::Return);
    assert!(!h.get_bold_on(), "a press does not toggle yet");
    key_up(h.window(), Key::Return);
    assert!(h.get_bold_on());
    tap(h.window(), " ");
    assert!(!h.get_bold_on());

    h.set_wrap_enabled(false);
    press(&h, "Wrap");
    assert_eq!(h.get_toggles(), 2);
    assert_eq!(one(&h, "Wrap").accessible_description().as_deref(), Some("Unavailable"));
    tab(h.window());
    assert!(!h.get_bold_focused(), "Tab moves on past the disabled toggle");
}

// One on at a time: a press replaces the selection, pressing it again clears it. With
// `multiple` presses add and remove. Each press reports the whole selection.
fn a_toggle_group_selects_one_or_many() {
    let h = harness();
    assert_eq!(one(&h, "Left").accessible_checked(), Some(true));
    press(&h, "Center");
    assert_eq!(strings(h.get_picked()), ["center"]);
    assert_eq!(strings(h.get_reported()), ["center"]);
    assert_eq!(one(&h, "Left").accessible_checked(), Some(false));
    press(&h, "Center");
    assert!(strings(h.get_picked()).is_empty());
    h.set_many(true);
    press(&h, "Left");
    press(&h, "Right");
    assert_eq!(strings(h.get_picked()), ["left", "right"]);
    press(&h, "Left");
    assert_eq!(strings(h.get_picked()), ["right"]);
    assert_eq!(h.get_reports(), 5);
}

// A labeled face is its word and icon 12 px inside a 1 px border; the group pads 2 px inside its
// own border and sets faces 2 px apart.
fn faces_size_to_their_words() {
    let h = harness();
    let (left, center) = (bounds(&one(&h, "Left")), bounds(&one(&h, "Center")));
    assert_eq!(center.w, 2.0 * (12.0 + 1.0) + 14.0 + 6.0 + h.get_center_width());
    assert_eq!(center.x, left.x + left.w + 2.0);
    assert_eq!(left.h, 28.0);
}

// Too narrow for every word, an all-icon group shows icons only from the next frame, each word
// a tooltip; a group with an item lacking an icon never does.
fn a_narrow_icon_group_goes_compact() {
    let h = harness();
    assert!(!h.get_compact());
    let full: f32 = ["Left", "Center", "Right"].iter().map(|l| bounds(&one(&h, l)).w).sum::<f32>() + 2.0 * 3.0 + 2.0 * 2.0;
    h.set_group_width(full);
    advance(16);
    assert!(!h.get_compact(), "every word fits at {full}");
    h.set_group_width(full - 1.0);
    advance(16);
    assert!(h.get_compact(), "one pixel short");
    h.set_group_width(120.0);
    advance(16);
    assert!(h.get_compact());
    let center = bounds(&one(&h, "Center"));
    assert_eq!(center.w, 28.0);
    mouse_move(h.window(), center.x + 14.0, center.y + 14.0);
    advance(600);
    assert!(
        ElementHandle::find_by_accessible_label(&h, "Center").count() >= 1,
        "the word is its tooltip and its name"
    );
    assert!(one(&h, "Plain").size().width > 28.0, "a word without an icon keeps the group in full");
}

cases!(toggles: a_button_group_joins_its_buttons, a_toggle_button_flips_on_each_press, keys_toggle_and_disabled_ignores, a_toggle_group_selects_one_or_many, faces_size_to_their_words, a_narrow_icon_group_goes_compact);
