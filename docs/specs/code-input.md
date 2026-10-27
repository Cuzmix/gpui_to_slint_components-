# Spec: CodeInput, JsonInput

Source: `ely-core/src/forms/code.rs` (`CodeInput`, `JsonInput`, `code_highlights`, `json_highlights`), `ely-core/src/syntax.rs`.

- CodeInput: a one-line Input in the mono face whose text is colored as typed with the generic C-like lexer: comments, strings, numbers, keywords, constants, function calls, type names, variables, punctuation, operators and properties each take their syntax color.
- JsonInput: an Input in the mono face, `rows-min` lines growing to `rows-max`, colored as JSON (keys as properties, strings, numbers, `true`/`false`/`null` as constants, punctuation). The text is parsed as typed; the first error ("expected ident at line 1 column 10") frames the field in danger and shows 4 px below in xs danger. A blank field is not an error.

## Slint mapping
- `CodeInput inherits Input` with the mono face and highlighting on; `JsonInput { text, placeholder, label, rows-min, rows-max, enabled; edited }`, out `has-focus`, `error`.
- Lexing, coloring and parsing run in Rust (`src/code.rs`, `src/syntax.rs`) through `TextFields.code-colors`, `json-colors` and `json-error`, wired by `wire_fields!`. The palette is the theme's syntax colors in `Tone` order.
- Colors are drawn over the editor (G53): a multi-line JsonInput, or a line wider than the field, shows plain.
