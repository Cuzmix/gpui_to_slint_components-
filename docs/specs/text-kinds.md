# Spec: text kinds (Title, Subtitle, Paragraph, Label, Caption, Overline, Heading)

Source: `ely-core/src/typography/text.rs`.

Each kind is a text with one size, weight, ink and leading (line height as a multiple of the font size). No states, events, keyboard or animation. Text wraps at word boundaries, as gpui's default text does.

| Kind | Size | Weight | Ink | Leading |
|---|---|---|---|---|
| Title | xl (20) | 600 | fg | 1.3 |
| Subtitle | lg (16) | 400 | fg_muted | 1.45 |
| Paragraph | md (14) | 400 | fg | 1.6 |
| Label | base (13) | 500 | fg | 1.4 |
| Caption | sm (12) | 400 | fg_muted | 1.45 |
| Overline | xs (11) | 600 | fg_subtle | 1.4, text set upper-case |
| Heading h1…h6 | display (32), xxl (24), xl (20), lg (16), md (14), base (13) | 600 | fg | 1.25 |

Sizes scale with `font_scale`. gpui snaps each line's height to whole device pixels, rounding half toward zero, so a 16 px Subtitle line is 23 px, not 23.2.

`Heading::new` panics on a level outside 1 to 6; the Slint `HeadingLevel` enum cannot hold one.

Slint: every kind `inherits TypeText`, which inherits `Text`, so all Text properties (overflow, alignment, color overrides) stay settable, as gpui's `Styled` refinement allows. `TypeText.leading` is gpui's relative line height. Slint's `line-height-factor` multiplies the face's natural line height, so TypeText divides by `Theme.font-line-ratio` (Inter 4.1: 2478/2048) after snapping the line as gpui does. Overline is a one-child layout because it shows `text` upper-cased. The Text's own accessible role (`text`) is kept; gpui gives headings no heading role either.

`tabular` and `literal` (font features) are not portable: gap G20.
