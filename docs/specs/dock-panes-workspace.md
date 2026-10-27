# Spec: Dock, PaneGroup, Workspace

Source: `ely-core/src/layout/dock/mod.rs`, `dock/view.rs`, `panes.rs`, `workspace.rs`, `split.rs` (`SplitPane`, `resize_handle`); stories in `ely-gallery` `pages/layout/systems.rs`; tests in `ely-core/src/layout/tests.rs` and the modules' own.

## Dock model
- Panels (id, title, icon) each sit at a spot: docked on a side (left, right, top, bottom) or floating at a window corner. Adding a repeated id panics. The first panel added to a side shows on it.
- Sizes start at 240 px for left and right (an open sidebar) and 160 px for top and bottom (`pane_min`), and are kept in rems (16 px).
- `activate(id)` shows a docked panel on its side; a floating one changes nothing. An unknown id panics.
- `move_panel(id, spot)`: if it was the shown panel of its old side, that side shows its first remaining panel (or none); a docked target shows it. Logged.
- `restore(layout)` is all or nothing. It refuses a panel placed twice, sizes that are not finite and positive, floating corners that are not finite, unknown panels (in spots or active), and an active panel not docked on that side. A side left unset shows its first panel.

## Dock view
- A row: left side, then a column (top side, center, bottom side), then the right side. A side with no shown panel is not drawn. Each side is its pane (size across, full length) plus a 6 px `resize_handle` toward the center. The center takes what is left (never below 0).
- Pane: clipped, `surface` fill; tab strip `control_height(Lg)` (32 px) with 4 px sides, 2 px gaps, a `border` hairline at the bottom; then the body.
- Tab: `control_height(Sm)` (24 px), 8 px sides, 6 px gap, radius md, text sm. The shown tab is `fg`, on `hover`, medium weight; the others `fg_subtle`. Hover turns the text (not the icon) `fg`. Icon sm in the tab's color. A click shows the panel. A drag (past gpui's 2 px threshold) carries a DragGhost (title and icon) held where it was grabbed.
- While a tab drags, five zones show over the dock: left (x 8, y 8, 20 % wide, height − 16), right (mirrored), top (from 22 % to 78 % across, y 8, 25 % tall), bottom (mirrored), float (30 % in from left and top, 40 % × 40 %). Each: radius lg, a 2 px dashed `border_strong` outline; under the pointer, `focus` at 12 % fill and `focus` at 70 % outline. Every zone under the pointer lights (gpui hovers every hitbox there); the drop goes to the top one (float, bottom, top, right, left) and stops. Float drops at the pointer's window position. Letting go anywhere ends the drag.
- A side's handle sets size = span from that edge of the dock, at least 160 px, at most 45 % of the dock that way (the cap wins), each move.
- Floating panels are FloatingPanel 240 × 380 titled by the panel, over the dock; a bar drag keeps the new corner without redrawing.

## PaneGroup
- A split tree of panes (ids from 1). `split(pane, axis)`: a direct child of a split along the same axis halves its share in place (shares normalized first); otherwise the pane becomes a two-way split. The new pane is focused. `None` (logged) for a gone pane.
- `close(pane)`: the last pane stays (logged); a gone one is logged. Shares renormalize (all zero becomes all 1); a split left with one child folds into it. A closed focused pane hands focus to the first pane.
- `focus(pane)`: an unknown pane is logged. `restore(layout)` validates first: unique ids, splits of two or more children with as many finite, non-negative sizes of positive sum; the next id must exist.
- View: each split is a SplitPane (min `pane_min`, 160 px): cells share by weight what the 6 px handles leave, edges on whole pixels; the handle line lights to `focus` on hover; a drag counts from its first move in shares of the split.
- Pane: fills its cell; `surface` when focused, else `bg`. Header 32 px, 12 px left, 4 px right, `border` hairline, text sm "Pane {id}" (`fg` focused, else `fg_subtle`); sm IconButtons panel-right (split right), panel-bottom (split down), x (close, disabled when it is the only pane), 2 px apart. A left press anywhere on the pane focuses it.

## Workspace
- `{ version: 1, dock: DockLayout, panes: PaneLayout }` as pretty JSON; serde's external tagging (`{"Docked":"Left"}`, `{"Floating":{"x":..,"y":..}}`, `{"Pane":1}`, `{"Split":{"axis":"Horizontal",..}}`). Unknown fields are ignored; a newer version is refused. `apply` validates the panes, restores the dock (all or nothing), then the panes.

## Slint
- The models are Rust (`ely_slint::dock`, `panes`, `workspace`), reached through the `Docks` and `PaneGroups` globals, wired by `wire_workspace!`, which returns the `WorkspaceHost`. Components name their model (`name`). The lists are kept models, synced row by row, so a press or drag survives a change.
- Rust lays out the pane tree once for every size: each edge is a share of the group plus fixed pixels, which the component rounds.
- Panel and pane bodies are drawn by the owner where the component says (G33). Zones are dashed by hand (G34). A body that takes presses focuses its pane itself (G35). The ghost and zones stay inside the dock, and floating panels clamp to the dock rather than the window (G12).
- Conservative readings: gpui panics on an unknown panel id from code; the wiring logs and ignores ids the UI cannot produce. The header buttons focus the pane on click (gpui on press, before the click), so a press dragged off a button does not focus the pane.
