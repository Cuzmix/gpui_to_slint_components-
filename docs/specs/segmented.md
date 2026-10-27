# Spec: SegmentedControl (+ Segment)

Source: `ely-core/src/buttons/segmented.rs`, `motion/slide.rs`, `layout/scroll.rs` (`reveal_when_focused`, `bring_into_view`).

- Props: id, selected (value), segments (value, label, optional icon), size (default md), on_change(value).
- Strip: sunken fill, 1 px border, radius lg, 2 px padding; scrolls sideways only.
- Control inside: grows and shrinks, at least control-height × (compact ? 1 : 2) × count wide; items centered.
- Segment: flex auto (content basis, grow and shrink), min width 0, control-height tall, control-padding each side, 1 px transparent border (focus-colored while focused), radius md, gap 6 px; label_size(size) text, semibold while chosen, medium otherwise; fg chosen, fg-muted otherwise; icon in the same color; label ends in an ellipsis.
- Role radio button, toggled = chosen, name label (value when empty). Every segment is a Tab stop; gpui click keys; a mouse press takes no focus. A press on the chosen segment does nothing; another reports on_change(value) and logs.
- Compact: every segment has an icon, last frame's strip width > 0 and < natural; icons only, the label a tooltip. Natural = 2 × (2 px + 1 px) + Σ (2 × (control-padding + 1 px) + label width in semibold + [icon: icon size + 6 px]).
- Thumb: under the segments, full control height, radius md, surface fill, 1 px border, raised shadow. On each change of choice it travels from the previous choice's span (as measured then) to the new one's over SLOW with `spring` (overshoots a little). No thumb until measured, or when `selected` matches no segment (logs an error).
- A segment that takes focus is brought into view with the least shift, its start first.

## Slint mapping
- `SegmentedControl { segments: [Segment]; in-out selected; size; selection-changed(value) }`; out `compact`, `scrolled`.
- The strip is a Rectangle around a Flickable; the control is a `FlexboxLayout` (no wrap, `alignment: stretch`), so segments grow by equal amounts and shrink in proportion to their widths as CSS flexbox does.
- The thumb: a second FlexboxLayout of the same segments, drawn below, sizes identically; its chosen cell draws the thumb, so the thumb passes under the neighbours. A cell that stops being chosen hands its span and `animation-tick()` to the control; the new one glides from there with `Theme.spring`.
- Compact and natural width as in ToggleGroup: last width from `changed width`, hidden measuring rows.
