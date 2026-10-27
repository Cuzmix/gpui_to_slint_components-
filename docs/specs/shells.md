# Spec: Sidebar, Sheet, Drawer, Page (PageHeader, PageContent), AppShell, MasterDetail

Source: `ely-core/src/layout/sidebar.rs`, `sheet.rs`, `page.rs`; stories in `ely-gallery` `pages/layout/shells.rs`.

## Sidebar
- A column that never grows or shrinks, full height, clipped, `bg` fill, `border` hairline on the right. Width 240 px open, 56 px collapsed (`sidebar_width`).
- Each change of `collapsed` restarts a `motion::BASE` (200 ms) ease-in-out-cubic animation from the other width to the new one; the first paint and a settled state are still.

## Sheet
- On a Backdrop placed at `side` (an edge; Center panics); the scrim press closes it.
- Panel: a column, 20 px gap, 24 px padding, `overlay` fill, `Modal` shadow; 380 px (`sheet_size`) across the edge and full along it.
- Title row: items centered, space between, 12 px gap: the title (lg, semibold) and a close IconButton (`x`), which closes it.
- Entrance: over `motion::BASE` with ease-out-cubic, the panel's outer margin on its edge runs from −380 px to 0 (it slides in from beyond the edge).

## Drawer
- On a Backdrop at the bottom; the scrim press closes it.
- Panel: full width, at most 85 % of the height, a column with 16 px gap, 24 px sides, 32 px bottom, top corners radius xl, `overlay` fill, `Modal` shadow. First the grab: 12 px above and 8 px below a 36 × 4 px fully round `border_strong` grip, grab cursor. Then the children.
- Pull: dragging the grab moves the panel down by `max(0, y − anchor)` (anchor: the first drag position). Release: when pulled more than 30 % of the panel's measured height it closes; it then springs from where it was let go back to 0 over `motion::BASE` ease-out-cubic.
- Entrance: over `motion::BASE` ease-out-cubic, the bottom margin runs from −380 px to 0.

## Page
- Column, 32 px gap, 40 px sides, 32 px top and bottom.
- Header: a row, items at the bottom, space between, 24 px gap, 24 px under the text, a `border` hairline at the bottom; a column (4 px gap) of the title (xxl, semibold) and optional subtitle (md, `fg_muted`); the actions in a row, centered, 8 px apart.
- Body: a column, 24 px gap.

## AppShell
- Fills its parent, `bg` fill, `fg` ink: the title bar, a row that takes the rest (may shrink to 0) of the sidebar and a content column (takes the rest, may shrink to 0), the status bar.

## MasterDetail
- A horizontal SplitPane with sizes 0.36 and 0.64 and the given minimum.

## Slint
- Parts again stand for slots: PageHeader (its children are the actions) and PageContent; AppShellBar, AppShellMain and AppShellContent. A text colour set on AppShell does not reach children (no cascade).
- Sidebar, the Sheet entrance and the Drawer spring time from `animation-tick()` where gpui restarts an animation; the Sheet and Drawer entrances, which run once, use `animate`.
- Sheet and Drawer inherit Backdrop, so they fill what they cover; put them last in the window. A Sheet `side` of center is read as right (the conservative reading of a panic).
- The Drawer's release is the grab's own release: Slint keeps the pointer on the grab while pressed, so it arrives wherever the pointer is let go, as gpui's mouse-up-out does.
- MasterDetail inherits SplitPane with those sizes; wire it as a SplitPane.
