# Spec: ExpressionInput

Source: `ely-core/src/forms/expression.rs`.

- A column 4 px apart. An md Input in the mono family with an "=" prefix in fg-subtle; invalid while a non-blank formula fails.
- Formulas: numbers (digits and dots), names (letters, digits, `_`), `+ - * / % ^`, parentheses, commas. Precedence: unary minus, `^` (right-associative), `* / %`, `+ -`. Functions abs, sqrt, round, floor, ceil (one value each), min and max (one or more). Variables are given by name. Problems are named: "division by zero" (also `%`), "x is not defined", "a ( is never closed", "f( is never closed", "f takes one value", "f needs values", "f is not a function", "unexpected …", "the formula ends early", "x is not a number", and "step has no finite value" once any step leaves the finite numbers.
- Colors (`expression_highlights`): numbers number, a name before `(` function, other names property, operators operator, `( ) ,` punctuation; nothing colored if it does not tokenize.
- Under the field, for a non-blank formula: `= value` (EN separators, up to 4 places, trailing zeros and dot dropped) in fg-muted, or the problem in danger; xs.

## Slint mapping
- `ExpressionInput { text <=>, variables: [FormulaVariable], placeholder, label, enabled; edited, accepted }`, out `has-focus`, `result: FormulaResult { shown, ok, text }`. The line is also the field's accessible description.
- `Formulas.result` and `Formulas.colors` run `ely_slint::expression` over f64 (`wire_formulas!`); variables arrive as f32 (G55). Colors draw through TextField's highlight layer (G53).
