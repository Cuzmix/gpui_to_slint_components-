# Spec: Button

Source: `ely-core/src/buttons/button.rs`; click rules from gpui `elements/div.rs`.

## Props
label, name (accessible name when label is empty), icon, trailing_icon, shortcut (gpui key syntax), variant (default Secondary), size (default Md), disabled, loading, full_width, focus_handle, group slot (internal).

## Variants (bg / hover / pressed / fg / border)
| Variant | bg | hover | pressed | fg | border |
|---|---|---|---|---|---|
| primary | accent | accent_hover | accent_hover.mix(on_accent, .12) | on_accent | none |
| secondary | surface | hover | active | fg | border |
| outline | none | hover | active | fg | border_strong |
| ghost | none | hover | active | fg | none |
| subtle | hover | active | active.mix(border_strong, .5) | fg | none |
| success | success | success.mix(fg, .14) | success.mix(fg, .24) | on_accent | none |
| danger | danger | danger.mix(fg, .14) | danger.mix(fg, .24) | on_accent | none |
| link | none | none (+ underline) | none | link | none |

## Layout
Height control-height(size); side padding control-padding(size), none for link; 6 px gap; label text sm/base/md for sm/md/lg, weight 500, ellipsized; icons xs/sm/md; shortcut hint 4 px left padding, weight 400, fg at 55 % alpha, spelled `⌘S` on Mac, `Ctrl+S` elsewhere. 1 px border; radius md (6 px), or left-only / none / right-only in a group, with middle and last losing their left border. Never grows; shrinks when a row cannot fit it; full_width stretches.

## States
- hover: bg hover (instant, no animation); pressed: bg pressed; link hover underlines.
- focused: border turns `focus` (1 px), transparent border at rest for borderless variants.
- disabled: opacity 0.45, cursor not-allowed, no hover/pressed, no Tab stop, accessible description "Unavailable".
- loading: spinner (ring, icon size, fg) centered over invisible content at the same width; ignores presses; keeps focus ring and Tab stop; default cursor.

## Events
`on_click`: left press and release inside; or Enter/Space pressed without modifiers and then Enter/Space released with no other key between and focus unchanged. Mouse down never focuses (prevent default).

## Accessibility
Role button, label = label or name.
