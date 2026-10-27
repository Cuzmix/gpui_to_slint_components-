# Spec: ConfirmButton (+ ConfirmMode)

Source: `ely-core/src/buttons/confirm.rs`.

- Props: id, label, armed_label (optional), mode (Hold or Twice), size (default md), on_confirm.
- Times: HOLD 1200 ms, ARMED 3000 ms, DONE 1400 ms. One pending timer: each new step replaces it; releasing a hold drops it.
- Look: outline tone, danger tone while armed; border danger while holding, focus color while focused; hover fill of the tone; text danger while holding, else the tone's fg; control height, control-padding sides, radius md, gap 6 px, medium weight, label_size(size); content clipped.
- Label: armed_label while armed, else "Press again to confirm" (hold) or "Click again to confirm" (twice); otherwise label. Role button, named by the label shown.
- Hold mode, mouse: a left press (not while done) starts holding; a danger fill at 16 % grows across, linearly, over HOLD; still holding at the end confirms. Release (inside or out) before that cancels. A mouse click does nothing.
- Click (mouse in twice mode, keyboard in either): ignored while done; armed: confirms; else arms for ARMED.
- Confirm: holding and armed end, done for DONE (presses ignored), a check icon in success turns in before the label over FAST (ease-out-cubic, from −0.6 rad); on_confirm runs.
- Tab stop with gpui's click keys; a mouse press takes no focus.

## Slint mapping
- `ConfirmButton { text; armed-text; mode; size; confirmed() }`; out `armed`, `holding`, `done`, `hold-progress`, `current-fill`, `current-border`.
- The one pending timer is one `Timer` and what it ends (`ConfirmExpiry`); the hold fill reads `animation-tick()`.
