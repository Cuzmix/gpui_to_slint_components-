//! Colored spans for fields that highlight as you type (gap G53), after ely-core
//! `forms/pattern.rs` (`parts`, `regex_highlights`).

use std::ops::Range;

use regex::Regex;
use slint::{Color, StyledText};

use crate::text::spans_markdown;
use crate::wire::{rgba, styled};

/// Token spans of a regex, by kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegexPart {
    Escape,
    Class,
    Group,
    Quantifier,
    Anchor,
    Alternation,
}

/// Splits a pattern into colored parts; plain text stays uncolored.
pub fn regex_parts(pattern: &str) -> Vec<(Range<usize>, RegexPart)> {
    let mut out = Vec::new();
    let mut chars = pattern.char_indices().peekable();
    while let Some((ix, ch)) = chars.next() {
        let span = |end: usize| ix..end;
        match ch {
            '\\' => {
                let end = chars.next().map_or(pattern.len(), |(next, ch)| next + ch.len_utf8());
                out.push((span(end), RegexPart::Escape));
            }
            '[' => {
                let mut end = pattern.len();
                while let Some((next, ch)) = chars.next() {
                    if ch == '\\' {
                        chars.next();
                    } else if ch == ']' {
                        end = next + 1;
                        break;
                    }
                }
                out.push((span(end), RegexPart::Class));
            }
            '(' | ')' => {
                let mut end = ix + 1;
                if ch == '(' && chars.peek().is_some_and(|(_, next)| *next == '?') {
                    chars.next();
                    end += 1;
                    if let Some((next, ch)) = chars.next_if(|(_, ch)| matches!(ch, ':' | '=' | '!' | '<')) {
                        end = next + ch.len_utf8();
                    }
                }
                out.push((span(end), RegexPart::Group));
            }
            '*' | '+' | '?' => out.push((span(ix + 1), RegexPart::Quantifier)),
            '{' => {
                let end = pattern[ix..].find('}').map_or(pattern.len(), |close| ix + close + 1);
                while chars.peek().is_some_and(|(next, _)| *next < end) {
                    chars.next();
                }
                out.push((span(end), RegexPart::Quantifier));
            }
            '^' | '$' => out.push((span(ix + 1), RegexPart::Anchor)),
            '|' => out.push((span(ix + 1), RegexPart::Alternation)),
            _ => {}
        }
    }
    out
}

/// Why `pattern` does not compile: the last line of the regex crate's message; none for an
/// empty or valid pattern.
pub fn regex_error(pattern: &str) -> Option<String> {
    if pattern.is_empty() {
        return None;
    }
    Regex::new(pattern).err().map(|error| error.to_string().lines().last().unwrap_or_default().trim().to_string())
}

/// The pattern colored by part; `palette` holds escape, class, group, quantifier, anchor and
/// alternation colors (keyword, string, function, number, constant, operator), the rest `plain`.
pub fn regex_colors(pattern: &str, palette: &[Color], plain: Color) -> StyledText {
    let color = |part: RegexPart| rgba(palette.get(part as usize).copied().unwrap_or(plain));
    let runs: Vec<_> = regex_parts(pattern).into_iter().map(|(range, part)| (range, color(part))).collect();
    styled(&spans_markdown(pattern, &runs, rgba(plain)), pattern)
}

#[cfg(test)]
mod tests {
    use super::{RegexPart, regex_error, regex_parts};
    use crate::text::spans_markdown;

    #[test]
    fn parts_cover_escapes_classes_groups_quantifiers() {
        let found = regex_parts(r"^(?:\d+)[a-z\]]{2,3}|x$");
        let kinds: Vec<RegexPart> = found.iter().map(|(_, part)| *part).collect();
        use RegexPart::*;
        assert_eq!(kinds, vec![Anchor, Group, Escape, Quantifier, Group, Class, Quantifier, Alternation, Anchor]);
        assert_eq!(found[1].0, 1..4);
        assert_eq!(found[5].0, 8..15);
    }

    #[test]
    fn errors_read_their_last_line() {
        assert_eq!(regex_error(""), None);
        assert_eq!(regex_error("a+"), None);
        assert_eq!(regex_error("(a").as_deref(), Some("error: unclosed group"));
    }

    #[test]
    fn spans_keep_spaces_and_escape() {
        let md = spans_markdown(" a*", &[(2..3, [1, 2, 3, 4])], [0, 0, 0, 255]);
        assert_eq!(md, "<font color=\"#000000ff\"> a</font><font color=\"#01020304\">\\*</font>");
    }
}
