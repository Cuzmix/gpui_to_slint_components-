# Spec: Ellipsis (Truncate), MiddleEllipsis, EllipsisTooltip, LineClamp

Source: `ely-core/src/typography/fit.rs`; LineClamp is gpui's `.line_clamp(n)` as `ely-gallery` `pages/typography/text.rs` shows it.

## Ellipsis
- One line in the text style around it (ambient: base 13 px, `fg`, weight 400, leading phi, left aligned). No wrapping.
- Its natural width is the whole text; it can shrink to 0 (`min_w_0`) and clips.
- When the whole text fits its box, it shows whole. Otherwise it shows the longest front that fits with `…` after it, trailing whitespace and `.,;:` dropped before the `…`. If even `…` does not fit, it shows nothing.
- The cut is aligned by the ambient alignment (left, center, right), never starting left of the box.
- `Truncate` is an alias of Ellipsis.

## MiddleEllipsis
- As Ellipsis, but the cut keeps `kept` characters around a middle `…`, the front taking the odd one, so both ends stay.

## EllipsisTooltip
- One line, size base, leading 1.4 (18 px), ink `fg`, gpui's `truncate()` (end ellipsis by the shaped line, no punctuation trimming).
- While the whole text is wider than the box, hovering shows the whole text in the standard tooltip (500 ms, hide on leave, press, scroll). While it fits, no tooltip.

## LineClamp
- Wrapped text kept to its first `n` lines, then cut: no ellipsis. Fewer lines keep their own height.

## Slint
- The cuts come from `TextFormat.end-cuts` / `middle-cuts` (host, `ely_slint::wire_text_format!`). The line measures every cut and shows the one that fits while the next does not, which is the cut gpui's binary search finds because widths never shrink. Unwired, it falls back to Slint's `overflow: elide` at the end.
- The ambient style is passed in as properties (`font-size`, `color`, `font-weight`, `leading`, `horizontal-alignment`) with the ambient defaults.
- `shown` reports what the line draws; `cut` whether it is cut.
- Accessibility: the line is one text node reading the whole text.
- Measured widths round up to whole pixels (G23).
