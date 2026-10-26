# Spec: Code, Blockquote, Kbd, KbdCombo, Highlight

Source: `ely-core/src/typography/inline.rs`, `typography/keys.rs`.

## Code
- A box 4 px padding left and right (none top and bottom) inside a 1 px `border` border, radius sm, fill `sunken`.
- Text: mono family, size sm (12), ink `fg`, gpui's default leading phi: one line is 19 px, the box 21 px.
- Sized to its text. gpui's box is a block, so in a column it would span the width; this port's Code always hugs its text, as it does in rows (the source's every use).

## Blockquote
- 2 px left border in `border_strong`, then 16 px padding before the passage.
- Passage ink `fg_muted`, size md (14), leading phi (23 px a line). Children follow one under another.
- A block: full width, as tall as its passage. Children do not inherit the style (G21).

## Kbd
- One keystroke in gpui key syntax; `secondary` is Command on the Mac, Control elsewhere.
- Caps in order, modifiers first: on the Mac ⌃ ⌥ ⇧ ⌘ fn then the key, with ↩ ⎋ ⇥ ⌫ ⌦ for named keys; elsewhere Ctrl, Alt, Shift, Win (Windows) or Super (Linux), Fn, and Enter, Esc, Tab, Backspace, Del. Arrows, Space, Home, End, PgUp, PgDn on all. Other keys upper-case.
- Caps 2 px apart. Each cap: min width xs × 1.8 (19.8 px), 4 px padding across, label centered, radius sm, 1 px border with a 2 px bottom edge in `border_strong`, fill `surface`, label xs (11) weight 500 in `fg_muted`. Height 18 + 3 = 21 px.
- The source panics on a keystroke it cannot parse. The port logs it and shows the source text on one cap: the most conservative reading that keeps the window running.

## KbdCombo
- Chords separated by whitespace, 8 px apart, centered on the cross axis.
- Inside a chord, caps 2 px apart; off the Mac, a `+` in `fg_subtle` between caps (ambient size base, 13 px).

## Highlight
- The text with every case-insensitive match of the query marked (regex-escaped, so `.` is literal); an empty query marks nothing; a query the regex engine refuses logs and marks nothing.
- `Highlight::new(text, ranges)`: marks given byte ranges.
- Mark: fill `warning_subtle`, ink `fg`; the rest takes the ambient style.
- Wrapping: gap G22.

## Slint
- Parsing keys and finding matches go through the `TextFormat` global's pure callbacks, wired by `ely_slint::wire_text_format!(app)`. Unwired: Kbd and KbdCombo show their source on one cap, Highlight shows the text unmarked.
- `Highlight.runs` takes precomputed runs (from `ely_slint::text::highlight_runs`) for given ranges.
- Accessibility: each cap is a text node reading its label; Highlight is one text node reading the whole text; Code and Blockquote passages are text nodes.
