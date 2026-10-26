//! Text work `.slint` cannot do: cutting strings, finding matches, grapheme clusters, per-letter
//! color and emoji names. Ported from ely-core `typography/{fit,inline,effects,emoji,rolling}.rs`.
//!
//! Slint measures and draws; these functions only produce strings. `TextFormat` in
//! `ui/components/text-format.slint` reaches them through pure callbacks the host wires with
//! [`wire_text_format!`](crate::wire_text_format).

use regex::RegexBuilder;
use unicode_segmentation::UnicodeSegmentation;

use crate::format::{self, Separators};

/// `kept` characters around a middle `…`: the front gets the odd one (ely `elide`).
pub fn elide(chars: &[char], kept: usize) -> String {
    let head = kept.div_ceil(2);
    let tail = kept / 2;
    let mut out: String = chars[..head].iter().collect();
    out.push('…');
    out.extend(&chars[chars.len() - tail..]);
    out
}

/// The front `kept` characters, trailing spaces and `.,;:` dropped, then `…` (ely `fit_end`).
pub fn cut_end(chars: &[char], kept: usize) -> String {
    let front: String = chars[..kept].iter().collect();
    let mut out: String = front
        .trim_end_matches(|ch: char| ch.is_whitespace() || matches!(ch, '.' | ',' | ';' | ':'))
        .into();
    out.push('…');
    out
}

fn distinct(cuts: impl Iterator<Item = String>) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for cut in cuts {
        if out.last() != Some(&cut) {
            out.push(cut);
        }
    }
    out
}

/// Every end cut, shortest first, each once. Their widths never shrink, so the line shows the
/// last one that fits; ely's binary search finds the same one.
pub fn end_cuts(text: &str) -> Vec<String> {
    let chars: Vec<char> = text.chars().collect();
    distinct((0..=chars.len()).map(|kept| cut_end(&chars, kept)))
}

/// Every middle cut, shortest first, each once.
pub fn middle_cuts(text: &str) -> Vec<String> {
    let chars: Vec<char> = text.chars().collect();
    distinct((0..=chars.len()).map(|kept| elide(&chars, kept)))
}

/// Byte ranges of every case-insensitive match of `query` (ely `Highlight::matching`).
pub fn matches(text: &str, query: &str) -> Vec<std::ops::Range<usize>> {
    if query.is_empty() {
        return Vec::new();
    }
    match RegexBuilder::new(&regex::escape(query))
        .case_insensitive(true)
        .build()
    {
        Ok(found) => found.find_iter(text).map(|found| found.range()).collect(),
        Err(error) => {
            log::error!("highlight: no marks for a {}-byte query: {error}", query.len());
            Vec::new()
        }
    }
}

/// `text` split into runs that alternate plain, marked, plain…; the first run is plain and may be empty.
pub fn highlight_runs(text: &str, ranges: &[std::ops::Range<usize>]) -> Vec<String> {
    let mut runs = Vec::new();
    let mut at = 0;
    for range in ranges {
        runs.push(text[at..range.start].to_string());
        runs.push(text[range.clone()].to_string());
        at = range.end;
    }
    if at < text.len() {
        runs.push(text[at..].to_string());
    }
    runs
}

/// Grapheme clusters, as a reader counts letters.
pub fn graphemes(text: &str) -> Vec<String> {
    text.graphemes(true).map(str::to_string).collect()
}

/// The first `count` grapheme clusters (ely `Typewriter`'s shown text).
pub fn grapheme_prefix(text: &str, count: usize) -> String {
    text.graphemes(true).take(count).collect()
}

/// An 8-bit channel blend, as ely's `Mix` lerps each channel then the color is stored.
pub fn mix(from: [u8; 4], to: [u8; 4], t: f32) -> [u8; 4] {
    if t <= 0.0 {
        return from;
    }
    if t >= 1.0 {
        return to;
    }
    let lerp = |a: u8, b: u8| (a as f32 + (b as f32 - a as f32) * t).round().clamp(0.0, 255.0) as u8;
    [lerp(from[0], to[0]), lerp(from[1], to[1]), lerp(from[2], to[2]), lerp(from[3], to[3])]
}

fn escape_markdown(grapheme: &str) -> String {
    let mut out = String::new();
    for ch in grapheme.chars() {
        match ch {
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '&' => out.push_str("&amp;"),
            '\\' | '`' | '*' | '_' | '{' | '}' | '[' | ']' | '(' | ')' | '#' | '+' | '-' | '.'
            | '!' | '|' | '~' => {
                out.push('\\');
                out.push(ch);
            }
            _ => out.push(ch),
        }
    }
    out
}

/// Markdown that gives each grapheme of `text` its own color.
pub fn tinted_markdown(text: &str, color: impl Fn(usize, usize) -> [u8; 4]) -> String {
    let spans = graphemes(text);
    let count = spans.len();
    let mut out = String::new();
    for (ix, grapheme) in spans.iter().enumerate() {
        let [r, g, b, a] = color(ix, count);
        out.push_str(&format!(
            "<font color=\"#{r:02x}{g:02x}{b:02x}{a:02x}\">{}</font>",
            escape_markdown(grapheme)
        ));
    }
    out
}

/// ely `GradientText`: `from` at the first grapheme to `to` at the last.
pub fn gradient_markdown(text: &str, from: [u8; 4], to: [u8; 4]) -> String {
    tinted_markdown(text, |ix, count| {
        mix(from, to, ix as f32 / count.saturating_sub(1).max(1) as f32)
    })
}

/// ely `ShimmerText` at loop position `t` (0..1): a band a quarter of the line wide sweeps
/// from -0.3 to 1.3, lighting `dim` toward `lit`.
pub fn shimmer_markdown(text: &str, dim: [u8; 4], lit: [u8; 4], t: f32) -> String {
    let band = -0.3 + (1.3 - -0.3) * t;
    tinted_markdown(text, |ix, count| {
        let x = ix as f32 / count.saturating_sub(1).max(1) as f32;
        mix(dim, lit, (1.0 - (x - band).abs() / 0.25).max(0.0))
    })
}

/// The emoji a shortcode names (`sparkles` → ✨), or none.
pub fn emoji(shortcode: &str) -> Option<&'static str> {
    emojis::get_by_shortcode(shortcode).map(|emoji| emoji.as_str())
}

/// ely `AnimatedNumber`'s text: grouped, `decimals` places, zero-padded to `pad` with the minus first.
pub fn rolling_text(value: f64, decimals: usize, pad: usize) -> String {
    let text = format::number(value, decimals, Separators::EN);
    match text.strip_prefix(format::MINUS) {
        Some(digits) => format!("{}{digits:0>1$}", format::MINUS, pad.saturating_sub(1)),
        None => format!("{text:0>pad$}"),
    }
}

/// Both texts right-aligned to one width with spaces, a character per place.
pub fn align(from: &str, to: &str) -> (Vec<String>, Vec<String>) {
    let width = from.chars().count().max(to.chars().count());
    let pad = |text: &str| {
        let mut chars = vec![" ".to_string(); width - text.chars().count()];
        chars.extend(text.chars().map(String::from));
        chars
    };
    (pad(from), pad(to))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn elide_keeps_both_ends() {
        let chars: Vec<char> = "abcdefghij".chars().collect();
        assert_eq!(elide(&chars, 4), "ab…ij");
        assert_eq!(elide(&chars, 5), "abc…ij");
        assert_eq!(elide(&chars, 0), "…");
        let wide: Vec<char> = "路径/很长/文件.rs".chars().collect();
        assert_eq!(elide(&wide, 4), "路径…rs");
    }

    #[test]
    fn end_cuts_drop_trailing_punctuation_and_repeat_nothing() {
        let cuts = end_cuts("Hi, you.");
        assert_eq!(cuts, ["…", "H…", "Hi…", "Hi, y…", "Hi, yo…", "Hi, you…"]);
    }

    #[test]
    fn middle_cuts_grow_from_both_ends() {
        assert_eq!(middle_cuts("abcd"), ["…", "a…", "a…d", "ab…d", "ab…cd"]);
    }

    #[test]
    fn matching_finds_every_case_insensitive_hit() {
        assert_eq!(matches("Ely, ely, ELY and élan", "ely"), [0..3, 5..8, 10..13]);
        assert!(matches("abc", "").is_empty());
        assert_eq!(matches("a.b ab", "."), [1..2]);
        assert!(matches("aaa", &"a".repeat(500_000)).is_empty());
    }

    #[test]
    fn runs_alternate_plain_and_marked() {
        let text = "Quiet interfaces age well. Loud ones AGE fast.";
        let runs = highlight_runs(text, &matches(text, "age"));
        assert_eq!(runs, ["Quiet interfaces ", "age", " well. Loud ones ", "AGE", " fast."]);
        assert_eq!(highlight_runs("age", &matches("age", "age")), ["", "age"]);
    }

    #[test]
    fn graphemes_keep_clusters_whole() {
        assert_eq!(graphemes("e\u{301}👍🏽!"), ["e\u{301}", "👍🏽", "!"]);
        assert_eq!(grapheme_prefix("e\u{301}👍🏽!", 2), "e\u{301}👍🏽");
    }

    #[test]
    fn gradient_runs_end_to_end() {
        let md = gradient_markdown("ab", [0, 0, 0, 255], [255, 255, 255, 255]);
        assert_eq!(md, "<font color=\"#000000ff\">a</font><font color=\"#ffffffff\">b</font>");
        let escaped = gradient_markdown("*", [0, 0, 0, 255], [0, 0, 0, 255]);
        assert_eq!(escaped, "<font color=\"#000000ff\">\\*</font>");
    }

    #[test]
    fn shortcodes_resolve_or_refuse() {
        assert_eq!(emoji("sparkles"), Some("✨"));
        assert_eq!(emoji("not_an_emoji_name"), None);
    }

    #[test]
    fn padding_keeps_the_minus_first() {
        assert_eq!(rolling_text(7.0, 0, 2), "07");
        assert_eq!(rolling_text(-12.0, 0, 5), "\u{2212}0012");
        assert_eq!(rolling_text(123.0, 0, 2), "123");
    }

    #[test]
    fn align_pads_by_place() {
        let (from, to) = align("999", "1,000");
        assert_eq!(from.concat(), "  999");
        assert_eq!(to.concat(), "1,000");
    }
}
