# Spec: Disclosure

Source: `ely-core/src/primitives/disclosure.rs`.

- A `chevron-right` icon, size xs unless set, color `fg_subtle` unless set.
- Shut: 0°. Open: 90° (points down).
- First paint: no animation, drawn at its end angle.
- Each change of `open`: turns over FAST (120 ms, 1 ms under reduced motion) with ease-out-cubic, from the other end angle to the new one.
- No events, keyboard or accessibility node (the owner's row is the control).

Slint: `Disclosure { open; size; color; }`, rotation animated with `cubic-bezier(1/3, 1, 2/3, 1)`, which is ease-out-cubic exactly. Interrupted turns: see PORTING_GAPS G14.
