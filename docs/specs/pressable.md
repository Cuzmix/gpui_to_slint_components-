# Spec: Pressable

Source: `ely-core/src/primitives/pressable.rs`.

- `pressed`: true from a left mouse down on it until the left button lifts, inside or outside.
- Long press: only when an `on_long_press` handler is set. 500 ms after the left press, if still pressed, it fires once and marks the press as long.
- `on_press`: fires when the left button lifts over it, still pressed, and no long press fired. A release outside fires nothing.
- Each new press resets the long-press mark. No keyboard, no focus, no animation, no accessibility node.

Slint: `Pressable { long-press-enabled; press => {} long-press => {} }`, children read `pressed`. Built from a TouchArea and a 500 ms Timer.
