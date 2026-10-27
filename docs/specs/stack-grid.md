# Spec: HStack, VStack, ZStack, AspectRatio, Container, SimpleGrid, Masonry

Source: `ely-core/src/layout/stack.rs`, `grid.rs`; stories in `ely-gallery` `pages/layout/basics.rs`.

## HStack / VStack (`h_stack()`, `v_stack()`)
- `h_stack`: a flex row with items centered on the cross axis. `v_stack`: a flex column (items stretch across, the CSS default). No gap unless the owner sets one; no wrapping.
- "Stack" is the same pair (the task list's alias).

## ZStack
- The first child is laid out normally and sets the stack's size; every later child is absolutely placed over it, filling it (`inset_0`).

## AspectRatio
- Width fills the parent; height is width / ratio; content past the box is clipped. A ratio that is not finite and positive panics.

## Container
- Width fills the parent up to the `ContainerSize` token (sm 640, md 960, lg 1200 px), centered (`mx_auto`), with 24 px padding left and right inside that width (taffy boxes include padding).

## SimpleGrid
- A CSS grid with `columns_for(width, min, gap)` equal columns: `floor((width + gap) / (min + gap))`, clamped to at least 1. Gap `gap` both ways. Rows size to their tallest item; items stretch to the row. `min` must be positive (panics otherwise).
- gpui measures the width after paint and lays out again on the next frame; the first frame has one column.

## Masonry
- `columns` equal columns (flex_1), `gap` apart, top aligned; each column a flex column with `gap` between items.
- Items, in order, join the column with the least filled height so far, ties to the left; each item adds its height plus `gap`. Heights are measured after paint (the first frame counts every height as 0, so items go round robin, then settle).
- Zero columns panics.

## Slint
- HStack and VStack are FlexboxLayout with `no-wrap` (row with `cross-axis-alignment: center`; column with the default stretch). They keep FlexboxLayout's properties, so `spacing` is the gap.
- ZStack is a one-cell GridLayout. The ground is its first plain child. Each covering layer goes in a `ZLayer`, which claims the same cell and reports no size of its own, so only the ground sizes the stack. A second plain child would take the next cell; this is the only way Slint can tell layers from the ground (`@children` cannot be split).
- AspectRatio is a clipping Rectangle whose height binds to its width. A non-positive ratio gives no height instead of panicking, which is the conservative reading of a panic in a UI library.
- Container centers a padded column of at most the token width inside its own width.
- SimpleGrid is a wrapping FlexboxLayout with `spacing: gap`. Slint cannot size a child from its container, so it exposes `columns` and `cell-width`, and each child binds `width: grid.cell-width`. Lines stretch items to the tallest, as grid rows do. The column count is right from the first frame; gpui's one-frame single-column start is not reproduced. A minimum below 1 px counts as 1 px.
- Masonry cannot move children between columns, so the owner passes `heights` and places each item with `item-x(i)`, `item-y(i)` and `column-width`. The assignment needs a loop, so it comes from `LayoutMath.masonry` (Rust, `ely_slint::wire_layout!`), which matches gpui's settled layout. Unwired, items go round robin at the top of their column. Fewer than one column counts as one.
- Center, Flex, Grid / GridItem, Inset / Padding, Wrap and Absolute / Positioned are Slint builtins (FlexboxLayout, GridLayout with `colspan`, layout `padding`, `flex-wrap`, `x`/`y`); the gallery shows each.
