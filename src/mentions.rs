//! What MentionInput cannot do in `.slint`: find the trigger before the caret, match handles,
//! splice a pick in and color mentions. Ported from ely-core `forms/mention.rs`.

use std::ops::Range;

use slint::{Color, StyledText};

use crate::text::spans_markdown;
use crate::wire::{rgba, styled};

/// How many handles show at once.
pub const SHOWN: usize = 6;

/// A character of a one-word handle.
pub fn one_word(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_'
}

/// The trigger that opens the word ending at `caret`, its query all `word` characters: its
/// byte offset and char.
pub fn active_trigger(text: &str, caret: usize, triggers: &[char], word: fn(char) -> bool) -> Option<(usize, char)> {
    let before = text.get(..caret)?;
    let start = before.char_indices().rev().find(|(_, ch)| ch.is_whitespace()).map_or(0, |(ix, ch)| ix + ch.len_utf8());
    let mut chars = before[start..].chars();
    let trigger = chars.next().filter(|ch| triggers.contains(ch))?;
    chars.all(word).then_some((start, trigger))
}

/// The handles that hold the typed query, ignoring case, a few at most. Handles that are not
/// one word are left out (ely panics on them).
pub fn handles_matching<'a>(handles: &'a [String], query: &str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    handles
        .iter()
        .filter(|name| {
            let ok = !name.is_empty() && name.chars().all(one_word);
            if !ok {
                log::error!("mention input: handle {name:?} must be one word");
            }
            ok
        })
        .filter(|name| name.to_lowercase().contains(&query))
        .take(SHOWN)
        .map(String::as_str)
        .collect()
}

/// `text` with `at..caret` replaced by `insert`, and the caret after it.
pub fn splice(text: &str, at: usize, caret: usize, insert: &str) -> (String, usize) {
    let (at, caret) = (at.min(text.len()), caret.min(text.len()));
    if !text.is_char_boundary(at) || !text.is_char_boundary(caret) || at > caret {
        return (text.to_string(), caret);
    }
    (format!("{}{insert}{}", &text[..at], &text[caret..]), at + insert.len())
}

/// Spans of `@name` and `#tag`.
pub fn mention_spans(text: &str) -> Vec<Range<usize>> {
    let mut spans = Vec::new();
    let mut previous_space = true;
    let mut chars = text.char_indices().peekable();
    while let Some((ix, ch)) = chars.next() {
        if previous_space && (ch == '@' || ch == '#') {
            let mut end = ix + 1;
            while let Some(&(at, next)) = chars.peek() {
                if !(next.is_alphanumeric() || next == '_') {
                    break;
                }
                end = at + next.len_utf8();
                chars.next();
            }
            if end > ix + 1 {
                spans.push(ix..end);
            }
            previous_space = false;
            continue;
        }
        previous_space = ch.is_whitespace();
    }
    spans
}

/// Mentions and tags in `link`, the rest `plain`.
pub fn colors(text: &str, link: Color, plain: Color) -> StyledText {
    let runs: Vec<_> = mention_spans(text).into_iter().map(|range| (range, rgba(link))).collect();
    styled(&spans_markdown(text, &runs, rgba(plain)), text)
}

/// Wires the `Mentions` global, which every MentionInput asks. The host's root `.slint` file
/// exports `Mentions`, `MentionTrigger`, `MentionAt`, `Choice` and `TextFit`.
///
/// ```ignore
/// ely_slint::wire_mentions!(app);
/// ```
#[macro_export]
macro_rules! wire_mentions {
    ($app:expr) => {{
        use ::slint::Model;
        let mentions = $app.global::<Mentions>();
        mentions.on_active(|text, caret, triggers| {
            let triggers: Vec<MentionTrigger> = triggers.iter().collect();
            let marks: Vec<char> = triggers.iter().filter_map(|t| t.mark.chars().next()).collect();
            let Some((at, mark)) = $crate::mentions::active_trigger(&text, caret.max(0) as usize, &marks, $crate::mentions::one_word) else {
                return MentionAt::default();
            };
            let handles: Vec<String> = triggers
                .iter()
                .find(|t| t.mark.chars().next() == Some(mark))
                .map(|t| t.handles.iter().map(|h| h.to_string()).collect())
                .unwrap_or_default();
            let query = &text[at + mark.len_utf8()..caret as usize];
            let rows: Vec<Choice> = $crate::mentions::handles_matching(&handles, query)
                .into_iter()
                .map(|name| Choice { value: name.into(), label: format!("{mark}{name}").into(), ..Default::default() })
                .collect();
            MentionAt { found: true, at: at as i32, mark: mark.to_string().into(), rows: ::slint::ModelRc::new(::slint::VecModel::from(rows)) }
        });
        mentions.on_splice(|text, at, caret, insert| {
            let (text, caret) = $crate::mentions::splice(&text, at.max(0) as usize, caret.max(0) as usize, &insert);
            TextFit { text: text.into(), caret: caret as i32 }
        });
        mentions.on_prefix(|text, at| text.get(..at.max(0) as usize).unwrap_or_default().into());
        mentions.on_colors(|text, link, plain| $crate::mentions::colors(&text, link, plain));
    }};
}

#[cfg(test)]
mod tests {
    use super::{active_trigger, handles_matching, mention_spans, one_word, splice};

    #[test]
    fn triggers_open_after_space_and_close_on_space() {
        let marks = ['@', '#'];
        let at = |text: &str| active_trigger(text, text.len(), &marks, one_word);
        assert_eq!(at("hi @ad"), Some((3, '@')));
        assert_eq!(at("@"), Some((0, '@')));
        assert_eq!(at("mail@ad"), None);
        assert_eq!(at("hi @ada lovelace"), None);
        assert_eq!(at("go #rel"), Some((3, '#')));
        assert_eq!(at("see @lift."), None, "a handle is one word");
    }

    #[test]
    fn handles_match_ignoring_case_six_at_most() {
        let names: Vec<String> = ["Ada", "adam", "bo", "x y", "a1", "a2", "a3", "a4", "a5"].iter().map(|s| s.to_string()).collect();
        assert_eq!(handles_matching(&names, "AD"), ["Ada", "adam"]);
        assert_eq!(handles_matching(&names, "a").len(), 6);
        assert!(!handles_matching(&names, "").contains(&"x y"), "not one word");
    }

    #[test]
    fn a_pick_replaces_the_trigger_and_query() {
        assert_eq!(splice("hi @ad there", 3, 6, "@ada "), ("hi @ada  there".to_string(), 8));
    }

    #[test]
    fn spans_cover_whole_names() {
        assert_eq!(mention_spans("ping @ada and #release now"), vec![5..9, 14..22]);
        assert!(mention_spans("a@b # c").is_empty());
    }
}
