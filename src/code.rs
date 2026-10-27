//! What CodeInput and JsonInput cannot do in `.slint`: split source into spans and color them
//! (gap G53), and check JSON. Ported from ely-core `forms/code.rs` (`lex`, `code_highlights`,
//! `json_highlights`).

use std::ops::Range;

use slint::{Color, StyledText};

use crate::syntax::Syntax;
use crate::text::spans_markdown;
use crate::wire::{rgba, styled};

/// What a span of source is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Comment,
    Text,
    Number,
    Word,
    Mark,
}

const KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "case", "catch", "class", "const", "continue", "def", "do",
    "else", "enum", "export", "fn", "for", "from", "func", "function", "if", "impl", "import",
    "in", "let", "match", "mut", "new", "pub", "return", "static", "struct", "switch", "throw",
    "try", "type", "use", "var", "while", "yield", "SELECT", "FROM", "WHERE", "JOIN", "LEFT",
    "RIGHT", "INNER", "OUTER", "ON", "AND", "OR", "NOT", "AS", "GROUP", "BY", "ORDER", "HAVING",
    "LIMIT", "OFFSET", "INSERT", "INTO", "VALUES", "UPDATE", "SET", "DELETE", "CREATE", "TABLE",
    "INDEX", "DROP", "ALTER", "DESC", "ASC", "DISTINCT", "UNION", "WITH", "CASE", "WHEN", "THEN",
    "ELSE", "END", "IS", "IN", "LIKE", "BETWEEN", "EXISTS", "INTERVAL",
];
const CONSTANTS: &[&str] = &[
    "false",
    "nil",
    "None",
    "null",
    "self",
    "this",
    "true",
    "undefined",
    "NULL",
    "TRUE",
    "FALSE",
];

/// Every language at once, for code of unknown kind.
pub const GENERIC: Syntax = Syntax {
    comment: "//",
    word_comment: false,
    keywords: KEYWORDS,
    constants: CONSTANTS,
};

/// Splits source into spans; spaces fall between them.
pub fn lex(source: &str) -> Vec<(Range<usize>, Kind)> {
    lex_with(source, &GENERIC)
}

/// Splits source as `syntax` reads it.
pub fn lex_with(source: &str, syntax: &Syntax) -> Vec<(Range<usize>, Kind)> {
    let mut out = Vec::new();
    let mut chars = source.char_indices().peekable();
    while let Some((start, ch)) = chars.next() {
        let kind = match ch {
            '"' | '\'' | '`' => {
                let mut escaped = false;
                while let Some((_, next)) = chars.next_if(|(_, next)| *next != '\n') {
                    if escaped {
                        escaped = false;
                    } else if next == '\\' {
                        escaped = true;
                    } else if next == ch {
                        break;
                    }
                }
                Kind::Text
            }
            _ if source[start..].starts_with(syntax.comment)
                && (!syntax.word_comment
                    || source[..start]
                        .chars()
                        .next_back()
                        .is_none_or(char::is_whitespace)) =>
            {
                while chars.next_if(|(_, next)| *next != '\n').is_some() {}
                Kind::Comment
            }
            '0'..='9' => {
                while chars
                    .next_if(|(_, next)| next.is_ascii_alphanumeric() || matches!(next, '.' | '_'))
                    .is_some()
                {}
                Kind::Number
            }
            ch if ch.is_alphabetic() || ch == '_' || ch == '$' => {
                while chars
                    .next_if(|(_, next)| next.is_alphanumeric() || matches!(next, '_' | '$'))
                    .is_some()
                {}
                Kind::Word
            }
            ch if ch.is_whitespace() => continue,
            _ => Kind::Mark,
        };
        let end = chars.peek().map_or(source.len(), |(next, _)| *next);
        out.push((start..end, kind));
    }
    out
}

/// The syntax colors code takes, in the order a palette lists them.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tone {
    Comment,
    String,
    Number,
    Keyword,
    Constant,
    Function,
    TypeName,
    Variable,
    Punctuation,
    Operator,
    Property,
}

/// The tone of each span of code as `lang` reads it.
pub fn code_tones(source: &str, lang: &Syntax) -> Vec<(Range<usize>, Tone)> {
    let spans = lex_with(source, lang);
    let next_is = |ix: usize, mark: &str| spans.get(ix + 1).is_some_and(|(range, _)| &source[range.clone()] == mark);
    spans
        .iter()
        .enumerate()
        .map(|(ix, (range, kind))| {
            let text = &source[range.clone()];
            let tone = match kind {
                Kind::Comment => Tone::Comment,
                Kind::Text => Tone::String,
                Kind::Number => Tone::Number,
                Kind::Word if lang.keywords.contains(&text) => Tone::Keyword,
                Kind::Word if lang.constants.contains(&text) => Tone::Constant,
                Kind::Word if next_is(ix, "(") => Tone::Function,
                Kind::Word if text.starts_with(char::is_uppercase) => Tone::TypeName,
                Kind::Word => Tone::Variable,
                Kind::Mark if "()[]{},.;".contains(text) => Tone::Punctuation,
                Kind::Mark => Tone::Operator,
            };
            (range.clone(), tone)
        })
        .collect()
}

/// The tone of each span of JSON: keys, strings, numbers, literals and marks; other words stay
/// plain.
pub fn json_tones(source: &str) -> Vec<(Range<usize>, Tone)> {
    let spans = lex(source);
    let keyed = |ix: usize| spans.get(ix + 1).is_some_and(|(range, _)| &source[range.clone()] == ":");
    spans
        .iter()
        .enumerate()
        .filter_map(|(ix, (range, kind))| {
            let tone = match kind {
                Kind::Text if keyed(ix) => Tone::Property,
                Kind::Text => Tone::String,
                Kind::Number => Tone::Number,
                Kind::Word if matches!(&source[range.clone()], "true" | "false" | "null") => Tone::Constant,
                Kind::Mark => Tone::Punctuation,
                Kind::Word | Kind::Comment => return None,
            };
            Some((range.clone(), tone))
        })
        .collect()
}

/// `source` colored by tone; `palette` lists the tones' colors in `Tone` order, the rest `plain`.
pub fn colored(source: &str, tones: &[(Range<usize>, Tone)], palette: &[Color], plain: Color) -> StyledText {
    let runs: Vec<_> = tones.iter().map(|(range, tone)| (range.clone(), rgba(palette.get(*tone as usize).copied().unwrap_or(plain)))).collect();
    styled(&spans_markdown(source, &runs, rgba(plain)), source)
}

/// The first parse error of non-blank JSON, as serde_json words it.
pub fn json_error(source: &str) -> Option<String> {
    if source.trim().is_empty() {
        return None;
    }
    serde_json::from_str::<serde_json::Value>(source).err().map(|error| error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kinds(source: &str) -> Vec<(&str, Kind)> {
        lex(source)
            .into_iter()
            .map(|(range, kind)| (&source[range], kind))
            .collect()
    }

    #[test]
    fn lexing_splits_strings_numbers_words_and_comments() {
        assert_eq!(
            kinds(r#"let s = "a \"b\""; // done"#),
            [
                ("let", Kind::Word),
                ("s", Kind::Word),
                ("=", Kind::Mark),
                (r#""a \"b\"""#, Kind::Text),
                (";", Kind::Mark),
                ("// done", Kind::Comment),
            ]
        );
        assert_eq!(
            kinds("f(0x1F, 2.5)"),
            [
                ("f", Kind::Word),
                ("(", Kind::Mark),
                ("0x1F", Kind::Number),
                (",", Kind::Mark),
                ("2.5", Kind::Number),
                (")", Kind::Mark),
            ]
        );
    }

    #[test]
    fn json_keys_read_as_properties() {
        let source = r#"{"a": [1, true]}"#;
        let tones: Vec<(&str, Tone)> = json_tones(source).into_iter().map(|(range, tone)| (&source[range], tone)).collect();
        assert_eq!(tones[1], ("\"a\"", Tone::Property));
        assert_eq!(tones[6], ("true", Tone::Constant));
        assert_eq!(json_error("{"), Some("EOF while parsing an object at line 1 column 1".into()));
        assert_eq!(json_error("  "), None);
    }

    #[test]
    fn calls_and_types_take_their_tones() {
        let source = "let x = Foo::new(1)";
        let tones: Vec<Tone> = code_tones(source, &GENERIC).into_iter().map(|(_, tone)| tone).collect();
        assert_eq!(tones, [Tone::Keyword, Tone::Variable, Tone::Operator, Tone::TypeName, Tone::Operator, Tone::Operator, Tone::Keyword, Tone::Punctuation, Tone::Number, Tone::Punctuation]);
    }

    #[test]
    fn an_open_string_stops_at_the_line_end() {
        assert_eq!(kinds("'open"), [("'open", Kind::Text)]);
        assert_eq!(kinds("\"a\n1"), [("\"a", Kind::Text), ("1", Kind::Number)]);
    }
}

