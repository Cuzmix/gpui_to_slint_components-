# Spec: Collapsible, Accordion, AccordionItem

Source: `ely-core/src/layout/collapse.rs`; stories in `ely-gallery` `pages/layout/regions.rs`.

## Collapsible
- A frame that clips. Its content's height is measured after paint.
- Settled (never changed, or the last change older than `motion::BASE`, 200 ms): open shows the content at its own height; shut shows nothing (the content is not rendered).
- On each change of `open`, an animation of `motion::BASE` with ease-in-out-cubic restarts: open grows the frame `height × t` from 0; shut shrinks it `height × (1 − t)` from the full height. A change mid-way restarts from that end.

## Accordion / AccordionItem
- Column with a `border` hairline on top. Each item: a `border` hairline below; a header and a Collapsible body.
- Header: a button (role button, label the title, `expanded` the state), a Tab stop with the focus ring (1 px border, transparent at rest), pointer cursor; row items centered, space between; 12 px top and bottom, 4 px left and right inside the 1 px border; the title at base size, medium weight; a Disclosure chevron at sm size in `fg_muted`.
- Body: 4 px left and right, 16 px bottom, in `fg_muted`.
- Toggling item i: it flips; every other item stays open only if `multiple`, or if it was open and i just shut. With `first_open`, item 0 starts open. Click, or Enter or Space on the focused header, toggles.

## Slint
- Collapsible reproduces the restart exactly: it times each change from `animation-tick()` and settles with a Timer, so it reads the tick only while moving. Shut and settled, the content is hidden (not drawn, no Tab stop).
- Accordion cannot hand state to children it did not make, so it keeps `open-items` (one bit per item) and offers `is-open(i)` and `toggle(i)`; each AccordionItem binds `open` and calls `toggle` from `toggled` (G32).
- AccordionItem's `text` draws a plain body in the muted ink; other children bind `ink` for it (no style cascade, as G21).
- The header uses ClickKeys for gpui's keyboard click, and offers the default and expand accessibility actions.
