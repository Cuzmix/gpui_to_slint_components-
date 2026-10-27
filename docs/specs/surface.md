# Spec: Card (CardHeader, CardBody, CardFooter), Panel (PanelHeader), Section (SectionHeader), Fieldset (Group), Frame, Well

Source: `ely-core/src/layout/surface.rs`; stories in `ely-gallery` `pages/layout/regions.rs`.

Every border is 1 px and, as taffy boxes count it, pads the content by 1 px. Texts take the ambient style (Inter, phi leading) unless named.

## Heading (private)
- A column, 2 px gap: the title at the given size, semibold, `fg`; then an optional line, sm, `fg_muted`. It may shrink (`min_w_0`).

## Card
- Column; radius lg, `border`, `surface` fill, no shadow.
- Header (optional): 20 px left, right and top; a row, items at the top, space between, 16 px gap: Heading (md) and one optional action.
- Body (always): 20 px all round; a column with 12 px gaps.
- Footer (optional): a `border` hairline on top, 20 px left and right, 12 px top and bottom; its content at the end, 8 px gap.

## Panel
- Column that clips; radius md, `border`, `surface` fill.
- Header bar: control height lg (32 px, density shifts it), 12 px left, 4 px right, a `border` hairline at the bottom, items centered, space between: the title (sm, semibold, `fg_muted`) and the actions in a row 2 px apart.
- Body: takes the rest (`flex_1`).

## Section
- Column, 16 px gap: a row (items at the bottom, space between, 16 px gap) of Heading (lg) and one optional action; then the body.

## Fieldset
- Column, 8 px gap: the legend (xs, semibold, `fg_muted`); then a column with 12 px gaps, 16 px in, radius lg, `border`.

## Frame / Well
- Frame: radius lg, `border_strong`, no fill, 16 px in.
- Well: radius lg, `sunken` fill, `border`, 16 px in.

## Slint
- Slint has one child slot, so the header, body and footer of a Card, the bar of a Panel and the title row of a Section are part components the owner places in order. Each part's children are what gpui hands it: CardHeader's are its action, CardFooter's its content, PanelHeader's its actions, SectionHeader's its action. A Card without a CardBody has no body padding, where gpui always draws an empty 40 px body; put an empty CardBody in to match.
- Panel's children after the PanelHeader share the rest of the height in a VerticalLayout, where gpui puts them in one block that takes it.
- Group is Fieldset (the task list's alias).
