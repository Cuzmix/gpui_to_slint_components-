# Spec: ChoiceChips (and FilterChips, its `multiple` form)

Source: `ely-core/src/forms/chips.rs`.

- A wrapping row, 8 px gap, a chip per choice. Chip: sm control height, 12 px sides, fully round, 1 px border, 6 px gap, sm text. On: accent fill and border, on-accent ink. Off: surface fill, border-strong border, fg-muted ink, hover fill while enabled. Disabled: fg-disabled ink, no press, not a tab stop.
- Enabled chips are tab stops with a focus ring; mouse down takes no focus; a click or the keyboard click presses. One chip replaces the choice; with `multiple` a chip toggles (choice order) and shows a check (xs) while on. The choice's icon (xs) sits before the label. Each press reports the values (logs). Roles: check box with `multiple`, radio button otherwise, named by the label, toggled.

## Slint mapping
- `ChoiceChips { choices, selected <=>, multiple; selection-changed(selected) }`. `selected` is in-out (G48). Edits run in Rust (`Choices.toggled`, `wire_options!`; `Toggles.contains`, `wire_toggles!`).
- The role must be constant in Slint, so filter chips and single chips are two chip kinds, one shown per mode.
