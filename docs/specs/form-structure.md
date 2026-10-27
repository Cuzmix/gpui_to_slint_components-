# Spec: Form, FormSection, FormField, FormLabel, FormError, InlineForm, DirtyIndicator

Source: `ely-core/src/forms/structure.rs`, `ely-core/src/forms/naming.rs`.

- Form: its children stacked 32 px apart. The secondary key and Enter (Command-Enter on the Mac, Control-Enter elsewhere) on any field or control inside submits it, once, before the control sees the key; the submit is logged. Without `on_submit` the key stays the controls'.
- FormSection: an h4 title with an optional caption 4 px below, then its fields, 16 px apart.
- FormLabel: the name (Label), then, 4 px apart and centered, a danger "*" when required and a help mark whose tip shows on hover.
- FormField: its FormLabel, the control, then the error (when set) or the description caption, 6 px apart. The control reads the label as its name and the error, else the description, as its description.
- FormError: an xs danger circle-alert icon, 4 px from the text in sm danger. Each new message eases in over 120 ms (ease-out-cubic), fading up from clear and settling from 4 px (`NUDGE`) above.
- InlineForm: its children in a row 12 px apart, bottoms aligned, wrapping when narrow.
- DirtyIndicator: while dirty, a warning dot (`status-dot`, 6 px) and "Unsaved changes" (or its label) 6 px apart, sm fg-muted, fading in over 200 ms (ease-out-cubic) each time it shows; nothing once clean.

## Slint mapping
- `Form inherits FocusScope { owner, can-submit; submitted() }`: the key is caught with `capture-key-pressed`. Slint reports the Mac's Command key as `control`, so `control` and Return is gpui's `secondary-enter`. `can-submit: false` stands for a form without `on_submit`. The log goes through `Forms` (`wire_forms!`).
- `FormField { label, required, help, description, error }`, out `note` (the error, else the description). The control is named by the owner: `Input { label: field.label; description: field.note; }` (G66).
- `FormSection { title, description }`, `FormLabel { text, required, help }`, `FormError { text }` (out `entrance`), `InlineForm` (a wrapping FlexboxLayout; children spell out their sizes), `DirtyIndicator { dirty, label }` (out `entrance`).
- FormDescription is Caption; FormActions is ActionBar; FieldArray is ported in `field-array.slint` (chapter 6a).
