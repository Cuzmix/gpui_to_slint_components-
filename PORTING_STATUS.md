# Porting status

One row per ported component. A component is done only when every column is ✓.
Visuals: compared state by state against the GPUI source values (no GPUI captures exist for Linux; the gallery's own captures are macOS-only), and checked in light and dark screenshots of the Slint gallery (`slint-viewer ui/gallery.slint --screenshot`).

| Component | Source | Props | Callbacks | Keyboard | Mouse/touch | Animation | Visuals | A11y | Tests | Gaps | Done |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Theme (tokens, palettes, fade) | theme/, motion/curve.rs | ✓ | n/a | n/a | n/a | ✓ (ThemeFader) | ✓ | n/a | ✓ `tests/theme.rs` | G1 G2 G4 | ✓ |
| Elevated | theme/tokens.rs `elevation` | ✓ | n/a | n/a | n/a | n/a | ✓ | n/a | ✓ (shadow tokens in `tests/theme.rs`) | G3 | ✓ |
| Icon | primitives/icon.rs | ✓ | n/a | n/a | n/a | n/a | ✓ | ✓ (no node) | ✓ `tests/primitives.rs` | | ✓ |
| Spinner | motion/spinner.rs | ✓ | n/a | n/a | n/a | ✓ periods, reduced motion | ✓ | ✓ progress indicator | ✓ `tests/primitives.rs` | G5 | ✓ |
| TooltipController / TooltipSurface | primitives/tooltip.rs (`Tooltip`) | ✓ | n/a | n/a | ✓ 500 ms, hide on leave/press/scroll | ✓ 120 ms ease-out rise | ✓ | n/a | ✓ `tests/icon_button.rs` | G6 G7 | ✓ |
| Button | buttons/button.rs | ✓ | ✓ | ✓ Enter/Space rules | ✓ | n/a (GPUI states are instant) | ✓ | ✓ | ✓ `tests/button.rs` | G8 G9 G11 | ✓ |
| IconButton, CloseButton, BackButton, MoreButton | buttons/icon_button.rs | ✓ | ✓ | ✓ | ✓ | n/a | ✓ | ✓ | ✓ `tests/icon_button.rs` | G10 G11 | ✓ |

Not started: everything else in `INVENTORY.md`, in its port order.
