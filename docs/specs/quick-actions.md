# Spec: QuickActions (+ QuickAction)

Source: `ely-core/src/buttons/quick.rs`.

- QuickAction: icon, label, note (optional), shortcut (gpui key syntax, spelled for the platform; panics on a bad one), run.
- QuickActions: panics with no action. A Tab stop (focus-colored border while focused), role menu; column 2 px apart, 4 px padding, radius lg, 1 px border, surface fill.
- Row: role menu item, named by its label, selected = chosen (and the active descendant). 12 px gap, 8 px sides, 6 px top and bottom, radius md, hover fill while chosen; pointer; hovering chooses it; a click runs it (logs).
- Icon box: a md control square, radius md, sunken fill, 1 px border, the icon sm in fg-muted. Text column fills: label base medium fg, note xs fg-subtle. Shortcut xs fg-subtle at the end.
- Keys (down): Down and Up step the choice, wrapping; Enter runs the chosen one; they stop propagation, other keys pass.

## Slint mapping
- `QuickActions { actions: [QuickAction]; activated(index) }`; out `selected`, `has-focus`.
- `shortcut` is already spelled, as Button's is (`ely_slint::shortcut_text`). No actions: an empty box (a panic in gpui).
- Roles list and list item for menu and menu item (G51).
