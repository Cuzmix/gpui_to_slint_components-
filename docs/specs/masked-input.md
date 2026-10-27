# Spec: MaskedInput, PhoneInput

Source: `ely-core/src/forms/masked.rs`.

- Mask slots: `9` a digit, `a` a letter, `*` either; anything else is literal. Typing keeps ASCII letters and digits, skips characters a slot rejects, writes literals as they come and stops with the input. An edit refits around the replaced range; the caret lands after the typed characters (or at the range start on a delete). While empty the placeholder is the mask with slots as underscores. A new mask refits what the old slots held (logs).
- MaskedInput: an Input with an optional prefix element.
- PhoneInput: countries US +1 🇺🇸 `(999) 999-9999`, GB +44 `9999 999999`, DE +49 `9999 9999999`, FR +33 `9 99 99 99 99`, JP +81 `99-9999-9999`, CN +86 `999 9999 9999`; panics on another code. Its prefix is the code: flag and dial, an xs chevron, 4 px apart, 4 px sides, radius sm, fg-muted, pointer, hover fill; a press keeps focus and toggles a list of the countries ("flag  code", dial as note) with the current one checked; a pick closes it, logs and reports the code; a press outside closes it.

## Slint mapping
- `MaskedInput { text, mask, label, size, enabled }` with children before the text. The fit runs in Rust as text-field kind `mask:<mask>` (`wire_fields!`); `TextFields.mask-hint` and `remask` give the hint and refit. A text set before wiring is fitted on the first timer tick.
- `PhoneInput { text, country, owner, label, size, enabled; country-changed(code) }`, out `picking`; `PhoneCountries` holds the table. An unknown code shows the first country (gpui panics). Its list needs an `OptionLayer` (G54); the outside press that closes it does not reach what lies under it.
