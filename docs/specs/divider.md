# Spec: Divider

Source: `ely-core/src/primitives/divider.rs`.

- Horizontal: a 1 px top border in `colors.border`, full width.
- Vertical: a 1 px left border in `colors.border`, full height.
- Label (horizontal only): rule, 12 px gap, text xs in `fg_subtle`, 12 px gap, rule; the two rules share the free width equally; items centered on the cross axis.
- No states, events, keyboard, animation or accessibility node.

Slint: `Divider { text; }` across, `VerticalDivider { }` down. The rule is a 1 px Rectangle (`Theme.hairline`).
