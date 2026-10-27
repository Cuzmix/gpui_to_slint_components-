# Spec: ButtonGroup, ToggleButton, ToggleGroup (+ ToggleItem)

Source: `ely-core/src/buttons/group.rs`.

## ButtonGroup
- A row of Buttons joined edge to edge (`flex`, no gap). Two buttons or more (panics otherwise).
- Each button gets a slot: first (left corners round), middle (square, left border hidden), last (right corners round, left border hidden).

## ToggleItem
- value (names it in a selection), label, icon, tooltip. A face needs a label or an icon (panics otherwise).

## Toggle face (`toggle_face`), shared
- Height control-height(size), at least as wide; side padding control-padding(size) when it shows a label; gap 6 px; radius md; 1 px transparent border that turns focus-colored while focused.
- Text: label_size(size) (sm/base/md), medium weight; fg while on, fg-muted while off; icon at the label's icon size in the same color.
- On: `active` fill. Off and enabled: `hover` fill on hover. No pressed fill.
- Role button, toggled = on; name: label, else tooltip, else value.
- Enabled: pointer cursor, Tab stop with gpui's click keys (Enter/Space released), a mouse press takes no focus, a click presses.
- Disabled: opacity 0.45, not-allowed cursor, description "Unavailable", no Tab stop, no press.
- Tooltip: the item's tooltip; compact faces show the tooltip or else the label.

## ToggleButton
- Props: id, item, on, disabled, size (default md), on_toggle(new state).
- A press calls on_toggle(!on) and logs.

## ToggleGroup
- Props: id, items, selected (values), multiple, size, on_change(selection).
- A press reports `pressed(selected, value, multiple)`: an item that is on turns off; one that is off joins with `multiple`, else replaces the selection.
- Frame: flex-wrap, items centered, 2 px gap, 2 px padding, radius lg, 1 px border.
- Compact: every item has an icon, last frame's width > 0 and < natural width. Natural = 2 × 2 px padding + 2 px border + 2 px × (n − 1) + Σ max(control-height, [label: 2 × control-padding + 2 px + label width at medium weight] + [icon: icon size + 6 px]).
- Faces are always enabled.

## Slint mapping
- `ButtonGroup inherits HorizontalLayout` (spacing 0); each Button sets `group-slot` (gap G47).
- `ToggleButton`: `in-out on`, flips on a press, then `toggled(on)`.
- `ToggleGroup`: `in-out selected: [string]`, set to the next selection on a press, then `selection-changed(selected)`. Membership and the next selection come from the `Toggles` global (`wire_toggles!`, `ely_slint::toggles::pressed`) (gap G48).
- Compact: `changed width` keeps the last width; a hidden row of measuring cells gives the natural width; a hidden column with a 1 px cell per icon-less item tells whether every item has an icon.
