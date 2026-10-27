# Spec: InlineEdit

Source: `ely-core/src/forms/inline.rs`.

- Shown: a row sm control high, sm control padding, 8 px gap, radius md, transparent 1 px border, focus ring, text cursor, hover fill. The value in an Ellipsis (fg), or the placeholder (`Empty`, fg-subtle) while empty; a pencil icon xs fg-subtle, visible only while hovered. Role text input named by `label`, valued by the value. A press does not take focus on mouse down; a click (or the keyboard click on its tab stop) opens it (logs).
- Open: an sm Input on the value, all of it selected and focused. Escape (key down) reverts and hands focus back to the text; Enter keeps on its release and hands focus back, so the release does not open it again. Leaving the field keeps the text and focus stays where it went. Keeping runs `on_commit(text)` (logs); reverting logs.

## Slint mapping
- `InlineEdit { value, label, placeholder; committed(text) }`, out `editing`, `has-focus`. `forward-focus` reaches the text's ClickKeys.
- The field stays instantiated and hidden while closed: a field inside `if` cannot take focus from its own `init`, so opening focuses it directly.
- `accessible-role` must be constant: the text keeps role text input and is hidden while the field is open.
