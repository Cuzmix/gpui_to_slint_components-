# Spec: Spinner

Source: `ely-core/src/motion/spinner.rs`.

- Props: style (ring default, dots, bars, pulse, orbit, wave), size (IconSize, default md), color (default `colors.fg_muted`).
- Periods: ring 900 ms, dots 1200, bars 1000, pulse 1400, orbit 1100, wave 1200; linear, repeating.
- Reduced motion: rests at t = 0.3.
- Geometry (side = min(w, h), stroke = side / 8, faint = color at 20 % alpha, `bump(t, lag) = 0.5 − 0.5 cos(2π(t − lag))`):
  - ring: faint full ring of radius side/2 − stroke; arc from −90° + 360°·t sweeping 360°·(0.2 + 0.3·bump(t, 0)), stroke width = stroke.
  - dots: 3 dots, radius side/9, x = center + (i − 1)·r·3.2, opacity 0.3 + 0.7·bump(t, i·0.18).
  - wave: same dots, full color, y = center + side·0.11 − side·0.22·bump(t, i·0.18).
  - bars: 4 bars, width side/8, height side·(0.3 + 0.55·bump(t, i·0.14)), x = center + (i − 1.5)·2w − w/2, fully rounded.
  - pulse: core dot radius side/6; ring radius core + (side/2 − core)·t, width 0.75·stroke, alpha 1 − t.
  - orbit: faint ring width 0.75·stroke; dot radius 1.1·stroke at angle −90° + 360°·t.
