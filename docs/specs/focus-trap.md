# Spec: FocusScope and FocusTrap

Source: `ely-core/src/primitives/focus.rs` (`FocusScope`).

- `FocusScope::new`: owns Tab and Shift-Tab for its subtree; untrapped, they move to the next or previous stop in the window, like the default.
- `.trap()`: while focus is inside, Tab on the last stop goes to the first and Shift-Tab on the first goes to the last. A trap with no stop keeps focus where it was (logs a warning). Tab from outside enters it normally, and a trap is passed over when it holds no stop.
- `.node(id, role, label, description)`: the scope reads as that role and label while it holds focus itself.
- `.root()`: the window's outermost scope takes focus when the focused element leaves the tree, or hands it back past overlays that left without closing.

Slint:
- An untrapped scope is Slint's own Tab chain: no component.
- `FocusTrap { trapping; }` inherits FocusScope (so the owner can set `accessible-role`/`accessible-label` for `.node`). A sentinel stop at each end catches focus leaving and asks `FocusSteps.step` to move one stop on; the host wires that to `ely_slint::focus::step`. See PORTING_GAPS G18.
- `.root()`: see PORTING_GAPS G19.
