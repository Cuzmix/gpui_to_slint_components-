# Spec: Icon

Source: `ely-core/src/primitives/icon.rs`, icons `ely-core/icons` (Lucide 1.48.0, 307 SVGs).

- Props: source (bundled name or app path), size (IconSize, default md = 16 px × font-scale), color (default `colors.fg`), rotate (radians about center), scale (glyph scale inside the same box), group hover color.
- Draws with one color only (SVG painted with the text color).
- App path that fails to load: logs and shows `image-off` in `danger`.
- No states, events, keyboard or animation. Not an accessibility node.

Slint: `Icon { source: Icons.check; size; color; rotation; scale; }`. Group hover color becomes the owner's binding of `color`.
