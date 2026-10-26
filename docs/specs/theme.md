# Spec: Theme (tokens, palettes, motion)

Source: `ely-core/src/theme/{mod.rs,tokens.rs,palette.rs,platform.rs}`, `ely-core/src/motion/curve.rs`.

## Props (Theme global)
| GPUI | Slint | Default |
|---|---|---|
| `mode` (`set_mode`, `set_mode_now`) | `mode`, `set-mode-now(m)` | light |
| `high_contrast` | `high-contrast` | false |
| `color_blind_safe` | `color-blind-safe` | false |
| `density` | `density` | standard |
| `radius_scale`, `font_scale` | `radius-scale`, `font-scale` | 1.0 |
| `font_family`, `mono_family` | `font-family`, `mono-family` | "Inter", "JetBrains Mono" |
| `reduced_motion` | `reduced-motion` | false |
| `platform` | `platform` | GPUI: build target (macOS → mac, Windows → windows, else linux). Slint: the same mapping from the builtin `Platform.os` |
| `set_palette(mode, Some/None)` | `use-custom-light/dark`, `custom-light/dark`, bump `custom-revision` | Ely's palettes |
| `colors` | `colors` (on screen), `palette` (target) | |

## Palettes
36 semantic colors, 8 chart hues, 16 ANSI, 13 syntax colors per mode, generated from `palette.rs` by `scripts/gen_palette.py`.
High contrast overrides `fg_muted`, `fg_subtle`, `border`, `border_strong`, and sets `glass` to `bg`. Color-blind-safe swaps only `chart`, and only for Ely's palettes; a custom palette is used as given.

## Fade
`fade_to_target`: every 8 ms frame, `t = elapsed / 280 ms` (1 ms under reduced motion), colors = `from.mix(to, ease_in_out_cubic(t))`, mixed per channel in sRGB. A new change restarts from the colors on screen. `set_mode_now` stops any fade.

## Measures
Control height sm/md/lg 24/28/32 px, ±4 px compact/comfortable. Control padding 8/12/16 px, ±2 px. Icon 12/14/16/20/24/32 px × font-scale. Text 11/12/13/14/16/20/24/32 px × font-scale. Radius 4/6/8/12 px × radius-scale. Every other measure in `tokens.rs` is a `Theme` property of the same name.

## Elevation (y, blur, alpha of `shadow`)
| Level | Light | Dark |
|---|---|---|
| raised | (1, 2, .06) | (1, 2, .4) |
| floating | (1, 2, .04) + (6, 16, .08) | (2, 4, .3) + (8, 24, .45) |
| modal | (2, 4, .04) + (16, 40, .14) | (4, 8, .3) + (20, 48, .55) |

## Motion
FAST 120 ms, BASE 200 ms, SLOW 320 ms, THEME 280 ms, NUDGE 4 px; all durations become 1 ms under reduced motion. `ease_out_cubic`, `ease_in_out_cubic` exact. `spring`: damping 0.75, ω = 6.91 / 0.75 per unit time.
