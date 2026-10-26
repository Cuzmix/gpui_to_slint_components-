# Spec: FocusRing

Source: `ely-core/src/primitives/focus.rs` (`FocusRing` trait).

- While the element holds focus its 1 px border turns `colors.focus`; at rest it keeps the element's own border color.
- A border, never a shadow.

Slint: `FocusRing { focused; rest-color; }` inherits Rectangle, so the owner sets radius and children.
