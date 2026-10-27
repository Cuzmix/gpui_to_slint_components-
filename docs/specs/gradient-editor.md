# Spec: GradientEditor

Source: `ely-core/src/forms/color/gradient.rs` (`GradientEditor`, `GradientStop`, `color_at`, `moved`, `added`).

- A column 8 px apart: a block padded 8 px left and right holding the bar and the stops 8 px apart, then a ColorPicker for the chosen stop.
- Bar: md control height (28 px), crosshair cursor, a checker (radius sm) under bands: the first stop's color from 0 to it, a gradient between each pair, the last color to 1; the first band rounded left, the last right (radius sm).
- A press on the bar adds a stop at the pointer, colored as the gradient is there (per-channel blend), and chooses it.
- Stops: a row one thumb (16 px) high; each stop a 16 px square, radius sm, 2 px border (chosen: focus while focused, else accent; others border-strong), filled with its color, centered on its place, grab cursor. A press chooses it; a drag (past 2 px) moves the chosen stop to the pointer (clamped 0..1), keeping the list sorted and following the stop.
- The stops row is a slider named by `label`, value round(place·100) of 0..100, described "Stop {n} of {count}"; focusable (a press focuses it). Left and Right move the chosen stop by 0.01; Delete or Backspace removes it while more than two remain, choosing the one before.
- The picker edits the chosen stop's color. Every change reports the stops (logged).
- Panics on fewer than two stops, or stops out of order or outside 0..1.

## Slint mapping
- `GradientEditor { owner, stops <=>, label; stops-changed(stops) }`, out `chosen`, `has-focus`. `stops` is in-out (G48).
- List edits run in Rust (`Gradients`: `added`, `moved`, `removed`, `recolored`, `stop-at`), wired by `wire_gradients!`; Slint cannot grow or reorder an array.
- One TouchArea over the stops (reaching the handles' outer halves) finds the handle under a press with `Gradients.stop-at`: the last drawn wins where handles overlap, as gpui's hit test does.
- Invalid stops are drawn as given; nothing panics.
