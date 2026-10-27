# Spec: IconPicker, EmojiPicker

Source: `ely-core/src/forms/glyphs.rs` (`IconPicker`, `EmojiPicker`, `emoji_found`), `ely-core/src/forms/picks.rs` (`finder`, `grid`, `rows`, `stepped`).

- A column 8 px apart, eight large-control cells wide: a search Input with a small fg-subtle search icon ("Search icons" / "Search emoji"), then a grid 280 px tall (`list-max-height`) that scrolls.
- The search is trimmed and lowercased. Icons match when their name, with spaces for dashes, holds it. Emoji match on name (any case) or shortcode; with no search they sit under their Unicode group titles (xs fg-subtle, at the bottom of a cell-high row, 4 px up).
- Rows are eight cells across. Each cell is a large-control square, radius md, with a 1 px ring: focus color on the cursor while the grid has focus, accent on the chosen glyph, clear otherwise; the chosen one is filled active, a hovered one hover. Icons draw md in fg; emoji lg. Hovering shows the glyph's name as a tooltip.
- Down in the field moves focus into the grid; Enter in the field picks the cursor's glyph. In the grid, Enter or Space picks it, and the arrows move it (left and right through the order, up and down keeping the column, landing on the last glyph of a shorter row, crossing titles), scrolling its row into view. A press puts the cursor on a cell and picks it. A new search puts the cursor first and scrolls to the top.
- Nothing found: "No icons match" / "No emoji match", centered 24 px down in fg-subtle, the grid's height.
- Picks log ("icon picker …: star") and go to `on_change`.

## Slint mapping
- `IconPicker { owner, selected <=> (an icon name); selection-changed(name) }` and `EmojiPicker { owner, selected <=> (the emoji); selection-changed(emoji) }`, out `text`, `cursor`. `selected` is in-out (G48).
- Both wrap the shared `GlyphGrid` (glyph-grid.slint). Finding, laying out rows and arrow steps run in Rust (`Glyphs`, `wire_glyphs!`, `src/glyphs.rs`); icons come from `Icons.all`.
- Only rows in view hold cells, as gpui's uniform list paints only those.
- The grid is a table of list items, its value the cursor's name (G60).
