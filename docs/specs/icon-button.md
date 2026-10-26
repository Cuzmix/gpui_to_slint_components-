# Spec: IconButton (+ close_button, back_button, more_button)

Source: `ely-core/src/buttons/icon_button.rs`.

- Props: icon, variant (default Ghost), size (default Md), disabled, tooltip, on_click.
- Square of control-height(size); radius md; 1 px border in the tone's border color; icon xs/sm/md in the tone's fg.
- Same hover/pressed fills, focus ring, click and keyboard rules as Button; no loading.
- Disabled: opacity 0.45, not-allowed cursor, no Tab stop, description "Unavailable"; the tooltip still shows.
- Accessible name: tooltip text, else the icon's Lucide name.
- close_button: `x`, tooltip "Close". back_button: `arrow-left`, "Back". more_button: `ellipsis`, "More".
