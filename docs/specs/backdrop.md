# Spec: Backdrop

Source: `ely-core/src/primitives/backdrop.rs`.

- A full-window scrim in `colors.backdrop`, raised above the page, fading in from opacity 0 over BASE (200 ms) with ease-out-cubic.
- A left mouse down on the scrim fires `on_dismiss`. The children occlude it, so presses on them do not dismiss.
- Place: center (children centered both ways), left/right (row, children at that edge, stretched to full height), top/bottom (column, children at that edge, stretched to full width).
- No keyboard or accessibility node of its own (dialogs add those).

Slint: `Backdrop { place; dismiss => {} }` with children. It fills its parent, so it goes last in the window: see PORTING_GAPS G12.
