# Spec: Tooltip

Source: `ely-core/src/primitives/tooltip.rs`; show rules from gpui `elements/div.rs` and `window.rs` (`prepaint_tooltip`).

- Body: max width 256 px, padding 8 × 4 px, radius md (6 px), fill `tooltip_bg`, text `tooltip_fg` at text sm (12 px), floating elevation. With meta: title, 8 px gap, meta at 60 % alpha.
- Entrance: opacity 0 → 1 and top margin 4 px → 0 over FAST (120 ms), ease-out-cubic.
- Placement: view origin at pointer + (1, 1) px; body 18 px (cursor offset) below that. Flips left/up when it would leave the window.
- Show: 500 ms (gpui default) after the first pointer move over the host; position fixed at show time.
- Hide: pointer leaves, any mouse down not on the tooltip, scroll. Touch long press shows it (gpui).
- TooltipTrigger: own delay (default 400 ms), optional follow-mouse; hides on any mouse down.
- HelpTooltip: `circle-help` icon xs in `fg_subtle` with a text tooltip.
