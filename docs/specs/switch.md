# Spec: Switch

Source: `ely-core/src/forms/switch.rs`.

- Track: switch-track (32 × 18), fully round, 1 px border (focus while focused, else transparent), 2 px padding, fill accent on and border-strong off; 0.5 opacity disabled. Thumb: a circle the content height, raised shadow, on-accent on and fg-muted off, at `travel × place` from the start, travel = track width − track height.
- After a change (not on first show), over motion-base: the thumb's place springs from the old end to the new (`motion::spring`, overshooting a little), its fill mixes linearly, and the track fill mixes linearly.
- A tab stop unless disabled; row 8 px gap, base text, fg (fg-disabled disabled), label beside taking the rest. A press on the row or the keyboard click reports the flipped state (logs). Mouse down takes no focus. Role switch, toggled, named by label or `name`; "Unavailable" when disabled.

## Slint mapping
- `SwitchMark { on, focused, enabled }`, `Switch { on <=>, label, name, enabled, unavailable-text; toggled(on) }`, out `has-focus`. `on` is in-out (G48).
