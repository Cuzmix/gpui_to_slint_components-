# Spec: MentionInput

Source: `ely-core/src/forms/mention.rs`.

- An md Input. Triggers (`@` people, `#` tags, …) each offer their handles; each handle must be one word (letters, digits, `_`; ely panics otherwise).
- A trigger is active when the word ending at the caret starts with it (after a space or at the start) and the rest of the word is one-word characters. Its rows: handles holding the query, ignoring case, at most 6, each labeled trigger + handle.
- While the field is focused and the trigger has rows, a floating list (`Popup`) shows under the trigger's character box. Up/Down move past disabled rows and wrap; Enter or a press replaces the trigger and query with trigger + handle + space as one undo step (logged); Escape hides the list until that trigger ends (a new trigger offers again).
- Highlights: `@name` and `#tag` spans after a space or the start, in the link color over an 8% link wash.

## Slint mapping
- `MentionInput { text <=>, owner, triggers: [MentionTrigger { mark, handles }], placeholder, label, size, enabled; edited, accepted }`, out `has-focus`, `active: MentionAt`, `open`, `highlighted`.
- `Mentions.active/splice/prefix/colors` run `ely_slint::mentions` (`wire_mentions!`); invalid handles are logged and left out. The list goes through `OptionAnchor`/`OptionLayer` (G54), anchored to a 1 px box at the trigger, measured from the text before it, as tall as the text line.
- The pick is a text set plus a caret move, so undo takes it back in Slint's own steps (G52). The colors draw through TextField's highlight layer (G53); the 8% wash is not drawn.
