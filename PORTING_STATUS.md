# Porting status

One row per ported component. A component is done only when every column is ✓.
Visuals: compared state by state against the GPUI source values (no GPUI captures exist for Linux; the gallery's own captures are macOS-only), and checked in light and dark screenshots of the Slint gallery (`slint-viewer ui/gallery.slint --screenshot`).

| Component | Source | Props | Callbacks | Keyboard | Mouse/touch | Animation | Visuals | A11y | Tests | Gaps | Done |
|---|---|---|---|---|---|---|---|---|---|---|---|
| Theme (tokens, palettes, fade) | theme/, motion/curve.rs | ✓ | n/a | n/a | n/a | ✓ (ThemeFader) | ✓ | n/a | ✓ `tests/theme.rs` | G1 G2 | ✓ |
| Elevated | theme/tokens.rs `elevation` | ✓ | n/a | n/a | n/a | n/a | ✓ | n/a | ✓ (shadow tokens in `tests/theme.rs`) | G3 | ✓ |
| Icon | primitives/icon.rs | ✓ | n/a | n/a | n/a | n/a | ✓ | ✓ (no node) | ✓ `tests/primitives.rs` | | ✓ |
| Spinner | motion/spinner.rs | ✓ | n/a | n/a | n/a | ✓ periods, reduced motion | ✓ | ✓ progress indicator | ✓ `tests/primitives.rs` | G5 | ✓ |
| TooltipController / TooltipSurface | primitives/tooltip.rs (`Tooltip`) | ✓ | n/a | n/a | ✓ 500 ms, hide on leave/press/scroll | ✓ 120 ms ease-out rise | ✓ | n/a | ✓ `tests/icon_button.rs` | G6 G7 | ✓ |
| Button | buttons/button.rs | ✓ | ✓ | ✓ Enter/Space rules | ✓ | n/a (GPUI states are instant) | ✓ | ✓ | ✓ `tests/button.rs` | G8 G9 G11 | ✓ |
| IconButton, CloseButton, BackButton, MoreButton | buttons/icon_button.rs | ✓ | ✓ | ✓ | ✓ | n/a | ✓ | ✓ | ✓ `tests/icon_button.rs` | G10 G11 | ✓ |
| Divider, VerticalDivider | primitives/divider.rs | ✓ | n/a | n/a | n/a | n/a | ✓ | ✓ (no node) | ✓ `tests/divider_pressable.rs` | | ✓ |
| Pressable | primitives/pressable.rs | ✓ | ✓ | n/a | ✓ 500 ms long press, release outside | n/a | n/a | ✓ (no node) | ✓ `tests/divider_pressable.rs` | | ✓ |
| Severity, Severities | primitives/severity.rs | ✓ | n/a | n/a | n/a | n/a | ✓ | n/a | ✓ `tests/primitives2.rs` | | ✓ |
| Disclosure | primitives/disclosure.rs | ✓ | n/a | n/a | n/a | ✓ 120 ms ease-out, still on first paint | ✓ | ✓ (no node) | ✓ `tests/primitives2.rs` | G14 | ✓ |
| DragGhost | primitives/ghost.rs | ✓ | n/a | n/a | n/a | n/a | ✓ | n/a | ✓ `tests/primitives2.rs` | G3 | ✓ |
| Picture (GPUI `Image`) | primitives/image.rs | ✓ | n/a | n/a | n/a | ✓ 1600 ms pulse, reduced motion | ✓ | ✓ (no node) | ✓ `tests/primitives2.rs` | G13 | ✓ |
| Backdrop, Place | primitives/backdrop.rs | ✓ | ✓ | n/a | ✓ dismiss outside children | ✓ 200 ms fade | ✓ | n/a | ✓ `tests/primitives2.rs` | G12 | ✓ |
| FocusRing | primitives/focus.rs | ✓ | n/a | ✓ | n/a | n/a | ✓ | n/a | ✓ `tests/primitives2.rs` | | ✓ |
| FocusTrap (FocusScope `.trap()`) | primitives/focus.rs | ✓ | n/a | ✓ wrap both ways, enter, pass over empty | n/a | n/a | n/a | ✓ (owner sets role/label) | ✓ `tests/focus_trap.rs` | G18 G19 | ✓ |
| Measure, IntersectionObserver | primitives/measure.rs | ✓ | ✓ | n/a | n/a | n/a | n/a | n/a | ✓ `tests/measure.rs` | G16 G17 | ✓ |
| HelpTooltip, TooltipTrigger | primitives/tooltip.rs | ✓ | n/a | n/a | ✓ 500 / 400 ms, hide on leave and press | ✓ (TooltipController) | ✓ | n/a | ✓ `tests/tooltip.rs` (rich content too) | G7 | ✓ |
| VisuallyHidden | tasks/ch01-10.md (GPUI shipped none) | ✓ | n/a | n/a | n/a | n/a | ✓ draws nothing | ✓ text role and label | ✓ `tests/primitives2.rs` | | ✓ |
| KeyStrokes (gpui `Keystroke::unparse`) | gpui platform/keystroke.rs | ✓ | n/a | ✓ | n/a | n/a | n/a | n/a | ✓ `tests/keystroke.rs` | | ✓ |
| Title, Subtitle, Paragraph, Label, Caption, Overline, Heading (TypeText) | typography/text.rs | ✓ | n/a | n/a | n/a | n/a | ✓ sizes, weights, inks, pixel-snapped leading | ✓ text role | ✓ `tests/typography.rs` | G20 | ✓ |
| Code | typography/inline.rs | ✓ | n/a | n/a | n/a | n/a | ✓ | ✓ text role | ✓ `tests/inline.rs` | G20 | ✓ |
| Blockquote | typography/inline.rs | ✓ | n/a | n/a | n/a | n/a | ✓ | ✓ text role | ✓ `tests/inline.rs` | G21 | ✓ |
| KeyCap, Kbd, KbdCombo | typography/inline.rs, keys.rs | ✓ | n/a | n/a | n/a | n/a | ✓ per platform | ✓ each cap reads its label | ✓ `tests/inline.rs` | G20 | ✓ |
| Highlight | typography/inline.rs | ✓ | n/a | n/a | n/a | n/a | ✓ | ✓ reads the whole text | ✓ `tests/inline.rs` | G22 | ✓ |
| Ellipsis (Truncate), MiddleEllipsis | typography/fit.rs | ✓ | n/a | n/a | n/a | n/a | ✓ cuts, alignment, nothing when no room | ✓ reads the whole text | ✓ `tests/fit.rs` | G23 | ✓ |
| EllipsisTooltip | typography/fit.rs | ✓ | n/a | n/a | ✓ tip only while cut | ✓ (TooltipController) | ✓ | ✓ reads the whole text | ✓ `tests/fit.rs` | G7 G23 | ✓ |
| LineClamp (gpui `.line_clamp`) | gpui text style | ✓ | n/a | n/a | n/a | n/a | ✓ hard cut | ✓ text role | ✓ `tests/fit.rs` | G29 | ✓ |
| Link, ExternalLink | typography/link.rs | ✓ | ✓ | ✓ Enter/Space rules, Tab stop | ✓ click, no focus on press, hover underline | n/a | ✓ | ✓ pressable, label | ✓ `tests/link.rs` | G9 G24 | ✓ |
| NumberText, CurrencyText, PercentText, FileSizeText, DateTimeText | typography/formatted.rs, format.rs | ✓ | n/a | n/a | n/a | n/a | ✓ (signed percent ink) | ✓ text role | ✓ `tests/values.rs` | G20 G25 | ✓ |
| DurationText, PluralText, RelativeTime (Clock, Figures) | typography/formatted.rs, format.rs | ✓ | n/a | n/a | n/a | ✓ 30 s refresh (host clock) | ✓ | ✓ text role | ✓ `tests/values.rs` (matches the Rust port case by case) | G20 G25 G26 | ✓ |
| CopyButton (chapter 5, ported early) | buttons/copy.rs | ✓ | ✓ | ✓ (IconButton) | ✓ | ✓ check turns in, 1.4 s | ✓ | ✓ (IconButton) | ✓ `tests/copy_select.rs` | | ✓ |
| CopyableText | typography/copy.rs | ✓ | n/a | ✓ (CopyButton) | ✓ | ✓ | ✓ | ✓ | ✓ `tests/copy_select.rs` | G20 | ✓ |
| SelectableText | typography/select.rs | ✓ | n/a | ✓ copy, select all | ✓ press, shift, drag | n/a | ✓ selection fill | ✓ text node | ✓ `tests/copy_select.rs` | G27 | ✓ |
| Emoji | typography/emoji.rs | ✓ | n/a | n/a | n/a | n/a | ✓ | ✓ text role | ✓ `tests/copy_select.rs` | | ✓ |
| Typewriter | typography/effects.rs | ✓ | n/a | n/a | n/a | ✓ per grapheme, caret breath, reduced motion | ✓ | ✓ text node | ✓ `tests/effects.rs` | | ✓ |
| GradientText, ShimmerText | typography/effects.rs | ✓ | n/a | n/a | n/a | ✓ 1.8 s sweep, reduced motion | ✓ | ✓ text node | ✓ `tests/effects.rs` | G28 | ✓ |
| AnimatedNumber | typography/rolling.rs | ✓ | n/a | n/a | n/a | ✓ roll and count, 640 ms ease-out | ✓ | ✓ reads its target | ✓ `tests/effects.rs` | G20 G25 | ✓ |

Chapter 1 entries that need no component of their own (Slint builtins or aliases) are marked in `INVENTORY.md`.

Not started: everything else in `INVENTORY.md`, in its port order.
