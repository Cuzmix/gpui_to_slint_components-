# Spec: CheckboxCard, RadioCard

Source: `ely-core/src/forms/card.rs`.

- A tab stop unless disabled. Row, items at the start, 12 px gap, 16 px padding, radius lg, 1 px border: focus while focused, accent while chosen, border; on hover (not chosen, not focused) border-strong. Surface fill. Disabled: 0.5 opacity, no press. Mouse down takes no focus.
- The mark (2 px down): a check mark (on/off) or a radio mark, never drawn focused, dimmed when disabled; motion replays on change.
- Words, taking the rest: title base medium fg; description 2 px under, sm fg-muted; preview 8 px under (RadioCard). An icon md fg-muted at the end.
- CheckboxCard: a press reports the flipped state (logs). RadioCard: a press while not selected runs `on_select` (logs). Roles check box / radio button, named by the title, toggled.

## Slint mapping
- `ChoiceCard { title, description, icon, enabled, unavailable-text, on, radio; pressed }` with `@children` as the preview. `CheckboxCard inherits ChoiceCard { checked <=>; toggled(on) }`, `RadioCard inherits ChoiceCard { chosen <=>; selected() }`; both in-out (G48). The description is also the accessible description.
- The mark and the card both dim when disabled, as in gpui (0.25 for the mark).
