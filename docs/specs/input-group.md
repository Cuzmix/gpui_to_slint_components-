# Spec: InputGroup, InputAddon

Source: `ely-core/src/forms/group.rs`.

- InputAddon: text (such as `https://`) or any element.
- InputGroup: a row, items centered, control height, clipped to radius md, 1 px border (focus while focused, else border-strong), surface fill, the field's text size and fg. A press anywhere focuses the field. Addons: full height, control padding each side, sunken, fg-muted, a 1 px border-strong hairline toward the field (right side before it, left side after). The field fills between them with control padding each side.

## Slint mapping
- `InputGroup { text, placeholder, label, before: [string], after: [string], size, enabled }`; text addons by list. Other addons are children wrapped in `InputAddon { before: false }`, after the field (one children slot, G47).
