# Spec: EyeDropper

Source: `ely-core/src/forms/color/dropper.rs` (`EyeDropper`).

- A ghost IconButton with the pipette icon. On macOS its tooltip reads "Pick a color from the screen" and a press opens AppKit's color sampler; the color picked reaches `on_pick` on the main thread (logged as hex). Giving up reports nothing.
- Elsewhere it is disabled, with the tooltip "Picking from the screen needs macOS".

## Slint mapping
- `EyeDropper { owner; picked(color) }`, out `available`, `has-focus`.
- The host samples the screen (G63): it sets `Colors.can-sample`, answers `Colors.sample(owner)`, and hands the color back with `ely_slint::deliver_sample!(app, owner, color)`, which reaches the dropper whose `owner` matches. Until `can-sample` is set the dropper shows disabled with the non-macOS tooltip.
