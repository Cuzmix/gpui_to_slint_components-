# Spec: ProgressBar / ProgressRing (chapter 6, ported early)

Source: `ely-core/src/motion/progress.rs`; stories in `ely-gallery` `pages/motion/progress.rs`.

- Glide: the first value shows at once; each change glides from the last target to the new value over `BASE` (200 ms) with ease-out-cubic (1 ms under reduced motion).
- ProgressBar: lanes `progress_thickness` (2 px) tall, round, clipped, on `border`. Determinate: `segments` lanes (default 1, at least 2 when set) 4 px apart, flexing equally; each fills its share of the value (`value × segments − index`, clamped) in `fg`, with the buffer's share behind it in `fg` at 25 %. A buffer shows only when it lies ahead of the value. Indeterminate: a 30 % fill sweeps from −30 % to 100 % over `SLOW × 4` (1280 ms), ease-in-out-cubic, repeating; under reduced motion it rests at 35 %.
- ProgressRing: `progress_ring` (48 px) unless sized; stroke the theme ring / 12 at any size; radius half the side less the stroke. Track a full `border` ring; fill an `fg` arc clockwise from the top, drawn only above 0. Percent: xs, tabular, `round(at × 100)%`. Content inside, centered; a percent or content, not both.
- gpui panics: value or buffer outside 0..1, fewer than two segments, a buffer on an indeterminate bar, a percent and content together. Here values are clamped, one segment is drawn, an indeterminate bar ignores its buffer, and a percent draws over content (G43).
