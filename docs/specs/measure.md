# Spec: Measure and IntersectionObserver

Source: `ely-core/src/primitives/measure.rs`.

- Measure: after each paint, if its bounds (window coordinates) differ from the last report, reports them once and draws again.
- IntersectionObserver: after each paint, compares its bounds with the clip it paints under (`content_mask`); reports `true` on first paint if visible, `false` if not, then only when that flips.
- Neither draws anything or takes events.

Slint: `Measure { measured(x, y, w, h) => {} }` reports on init and when `absolute-position`, width or height change. `IntersectionObserver { visible-x/-y/-width/-height; visibility-changed(seen) => {} }` with `out in-view`; the owner passes the visible area: see PORTING_GAPS G17.
