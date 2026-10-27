# Spec: Link, ExternalLink

Source: `ely-core/src/typography/link.rs`.

- A row, items centered, 2 px gap, 2 px padding across, radius sm, 1 px border: transparent at rest, `focus` while focused (FocusRing).
- Text in `colors.link`, the ambient size (base) at phi leading: 21 + 2 = 23 px tall. Pointer cursor.
- Hover underlines the text (gap G9 for the underline's placement); the arrow is not underlined.
- A Tab stop. A mouse press does not take focus (gpui prevents default on mouse down).
- A left click runs the handler; so do Enter or Space, pressed without modifiers and released, with nothing between (gpui's keyboard click, `ClickKeys`).
- Accessibility: role link, label the text. Slint has no link role (G24).
- ExternalLink: a Link with an `arrow-up-right` icon (xs, 12 px, link color) after the text; a press opens its address with the system handler (`cx.open_url`).
- Sizing: the source's box is a block-level flex row. Every story places it in a row, where it hugs its content; this port always hugs its content.

Slint: `Link { text; clicked => {} }`, `ExternalLink { text; url; }`. Opening needs the host: the `Links.open(url)` callback, wired by `ely_slint::wire_links!(app)` to `ely_slint::open::open_url`. Unwired, an external link does nothing but run `opened`.
