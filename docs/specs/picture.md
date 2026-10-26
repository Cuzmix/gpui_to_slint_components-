# Spec: Picture (GPUI `Image`)

Source: `ely-core/src/primitives/image.rs`.

- Fills the box its owner gives it; clips to its corner radii. Fit: cover unless set.
- Placeholder (no source): sunken pane, `image` icon lg in `fg_subtle`, centered.
- Loading: sunken pane whose opacity pulses with gpui `pulsating_between(0.55, 1.0)` over a 1600 ms repeating cycle; still (opacity 1) under reduced motion.
- Failed: sunken pane with the `image-off` icon lg in `fg_subtle`.
- No events, keyboard or accessibility node.

Slint: `Picture { source; has-source; loading; fit; }`, named so it does not clash with the builtin `Image`. `Theme.pulsating-between` ports gpui's curve. Loading is owned by the host: see PORTING_GAPS G13.
