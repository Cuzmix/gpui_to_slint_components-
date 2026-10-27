# Spec: RegexInput

Source: `ely-core/src/forms/pattern.rs`.

- A column 4 px apart. An Input (default md, `size` sets it) in the mono family, between a "/" prefix and a "/" suffix in fg-subtle; invalid (danger frame) while a non-empty pattern fails to compile.
- The text is colored by part (`regex_highlights`): escapes (`\x`) keyword, classes (`[...]`, escapes inside) string, groups (`(`, `(?`, `(?:`, `(?=`, `(?!`, `(?<`, `)`) function, quantifiers (`*`, `+`, `?`, `{...}`) number, anchors (`^`, `$`) constant, alternation (`|`) operator; the rest plain.
- Under the field, while the pattern fails: the last line of the regex crate's error, trimmed, xs, danger.

## Slint mapping
- `RegexInput { text <=>, placeholder, label, size, enabled; edited, accepted }`, out `has-focus`, `error`. The error is also the field's accessible description.
- `TextFields.regex-colors` and `regex-error` (wired by `wire_fields!`, `ely_slint::highlight`) color and check it. The colors are drawn by TextField's highlight layer (gap G53): while the text fits the field, the editor's text and caret go clear and the colored text and a focus-colored caret (530 ms blink) are drawn over them; text wider than the field shows plain.
