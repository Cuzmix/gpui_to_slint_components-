# Spec: CopyButton, CopyableText, SelectableText, Emoji

Sources: `ely-core/src/buttons/copy.rs`, `typography/copy.rs`, `typography/select.rs`, `typography/emoji.rs`.

## CopyButton (chapter 5, ported early for CopyableText)
- A 24 × 24 slot (control height sm), content centered, tooltip "Copy" (standard 500 ms tooltip on the slot).
- At rest: a small ghost IconButton with the `copy` icon. A press writes the text to the clipboard and logs the character count.
- For 1.4 s after: a `check` icon (sm, 14 px, `success`) instead of the button, turning from −0.6 rad to 0 over the fast motion (120 ms, ease-out cubic; 1 ms under reduced motion). The tooltip reads "Copied". The check takes no press.

## CopyableText
- A row, 4 px gap, items centered: the text in the ambient style, then a CopyButton for the same text.
- `mono` sets the monospace face. `tabular` (tabular figures) is not portable: G20.

## SelectableText
- Read-only text, wrapped, in the ambient style. Pointer: press places the caret (and focuses), shift-press extends, drag extends; I-beam cursor. Keys while focused: the platform's secondary key with C copies a non-empty selection, with A selects everything. No other keys act.
- The selection is drawn on `selection`; the text keeps its color.
- A new text clears the selection.
- Accessibility: role paragraph, label the text.

## Emoji
- One glyph in the system color font at text size lg (16 px) unless sized. `Emoji::shortcode` looks up a GitHub shortcode (the `emojis` crate) and is `None` for an unknown name.

## Slint
- The clipboard is reached through a never-drawn TextInput's `copy()` (`ClipboardWriter`), since `.slint` has no clipboard call.
- SelectableText is a read-only TextInput with the caret hidden and moving keys swallowed: G27.
- Emoji shortcodes go through `TextFormat.emoji`; an unknown or unwired shortcode shows nothing (`known` is false).
- Accessibility: Slint has no paragraph role, so SelectableText is a text node.

- Sizes as a layout: place it in a layout, or it fills its parent (G29).
- The software renderer draws emoji as monochrome fallback glyphs (G30).
