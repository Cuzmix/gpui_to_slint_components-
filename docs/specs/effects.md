# Spec: Typewriter, GradientText, ShimmerText, AnimatedNumber

Sources: `ely-core/src/typography/effects.rs`, `typography/rolling.rs`.

## Typewriter
- Reveals the text one grapheme cluster at a time at `speed` a second (32 by default): after a share `t` of the reveal, `ceil(count × t)` graphemes show. The whole reveal takes count / speed seconds, 1 ms under reduced motion. It restarts when the text changes.
- After the text: 2 px gap, then a caret 0.15 × 1.3 of the base text size (1.95 × 16.9 px) in `fg`, items centered. The caret's opacity breathes between 0.15 and 1 every 1.1 s (gpui `pulsating_between`), still at 1 under reduced motion. It keeps blinking after the text is out.
- The source panics on a speed of zero or less; the port shows the whole text at once.

## GradientText
- Each grapheme blends from `fg` at the first to `fg_subtle` at the last (or two given colors), in the ambient style. One grapheme takes the first color.

## ShimmerText
- The text in `fg_subtle`; every 1.8 s a band a quarter of the line wide sweeps from −0.3 to 1.3 of its length, lighting graphemes toward `fg` by `max(0, 1 − |x − band| / 0.25)`. Under reduced motion it holds still in `fg_subtle`.

## AnimatedNumber
- Shows `value` grouped, at `decimals` places, zero-padded to `pad` characters with the minus first. Size xxl (24 px), line 1.2 × size rounded to whole device pixels (29 px), one line tall, clipped. Ink `fg` unless set.
- On each new value it moves from the last target to the new one over twice the slow motion (640 ms; 1 ms reduced), ease-out cubic.
- Roll (default): both texts are right-aligned by place; each place that lands on a digit is a column of 0 to 9, one line each, that scrolls from the old digit (0 if the old place was not a digit) to the new one. Other places show the new character.
- Count up: the text of `from + (to − from) × eased` each frame.

## Slint
- Graphemes, colors per grapheme, number text and place alignment come from `TextFormat` (host). Unwired: the whole text, plain text in the first color, Slint's `to-fixed` without motion.
- Per-grapheme colors are a Slint `StyledText` built from markdown `<font color>` runs. StyledText takes no line height: G28.
- Timing reads `animation-tick()`, so Slint redraws only while something moves.
- Accessibility: one text node each; AnimatedNumber reads the value it lands on.
