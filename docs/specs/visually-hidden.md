# Spec: VisuallyHidden

Source: `tasks/ch01-10.md` (entry "VisuallyHidden — prove"); gallery story `primitives/parts.rs` `visually_hidden`.

- GPUI ships no component: the story shows a blocked note because gpui's AccessKit tree was not proven to reach a screen reader.
- The task defines it: a box that draws nothing, with a label, that reaches the accessibility tree with that label.

Slint: `VisuallyHidden { text; }`, 0 × 0, `accessible-role: text`, `accessible-label: text`. It takes no space in a layout and no events.
