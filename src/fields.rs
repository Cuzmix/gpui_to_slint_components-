//! What text fields cannot do in `.slint`: walk an edit's characters. Ported from ely-core
//! `forms/text/mod.rs` (`admit`, `replace`).
//!
//! The builtin `TextInput` applies an edit before the field sees it, so [`admit`] finds the edit
//! again from the text before and after, then filters, caps and fits it as gpui does. The
//! `TextFields` global in `ui/components/text-field.slint` reaches it through a pure callback the
//! host wires with [`wire_fields!`](crate::wire_fields).

use std::{
    cell::RefCell,
    collections::HashMap,
    ops::Range,
    rc::Rc,
    sync::LazyLock,
};

use regex::Regex;

static EMAIL: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").expect("email pattern compiles"));
static URL: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^https?://[^\s/?#]+\.[^\s/?#]+\S*$").expect("url pattern compiles"));

/// Whether `text` reads as one email address (ely `is_email`).
pub fn is_email(text: &str) -> bool {
    EMAIL.is_match(text)
}

/// Whether `text` reads as a web link (ely `is_url`).
pub fn is_url(text: &str) -> bool {
    URL.is_match(text)
}

/// Reshapes an edit: the text before it, the byte range replaced and the typed text in; the text
/// and the caret's byte offset out (ely `TextInput::set_fit`).
pub type Fit = Rc<dyn Fn(&str, Range<usize>, &str) -> (String, usize)>;
/// Whether a typed character stays (ely `TextInput::filter`).
pub type Filter = Rc<dyn Fn(char) -> bool>;

#[derive(Default)]
struct Kinds {
    filters: HashMap<String, Filter>,
    fits: HashMap<String, Fit>,
}

thread_local! {
    static KINDS: RefCell<Kinds> = RefCell::new(Kinds::default());
}

/// Names a filter for fields whose `fit-kind` is `kind`.
pub fn register_filter(kind: &str, allow: impl Fn(char) -> bool + 'static) {
    KINDS.with(|kinds| kinds.borrow_mut().filters.insert(kind.into(), Rc::new(allow)));
}

/// Names a fit for fields whose `fit-kind` is `kind`; a kind may have a filter too.
pub fn register_fit(kind: &str, fit: impl Fn(&str, Range<usize>, &str) -> (String, usize) + 'static) {
    KINDS.with(|kinds| kinds.borrow_mut().fits.insert(kind.into(), Rc::new(fit)));
}

/// The edit that turned `before` into `after`, leaving the caret at byte `caret` of `after`: the
/// byte range replaced in `before`, and what took its place. The caret settles runs of one
/// character ("aa" to "aaa"): what was typed ends at it.
pub fn edit_between<'a>(before: &str, after: &'a str, caret: usize) -> (Range<usize>, &'a str) {
    let caret = floor_boundary(after, caret.min(after.len()));
    // The common end first, no further back than the caret, then the common start in what is left.
    let mut tail = 0;
    for (a, b) in before.chars().rev().zip(after[caret..].chars().rev()) {
        if a != b {
            break;
        }
        tail += a.len_utf8();
    }
    // A run that crosses the caret cannot be told apart from the typed text: keep it in the end.
    let tail = tail.min(before.len());
    let (left, right) = (&before[..before.len() - tail], &after[..after.len() - tail]);
    let head = left
        .char_indices()
        .zip(right.chars())
        .find(|((_, a), b)| a != b)
        .map_or(left.len().min(right.len()), |((at, _), _)| at);
    (head..before.len() - tail, &after[head..after.len() - tail])
}

fn floor_boundary(text: &str, mut at: usize) -> usize {
    while !text.is_char_boundary(at) {
        at -= 1;
    }
    at
}

/// The text and caret after `before` became `after`, its caret at byte `caret`, in a field of
/// `kind`, capped at `max_len` characters (0: no cap), one line unless `multi_line`.
pub fn admit(before: &str, after: &str, caret: usize, kind: &str, max_len: usize, multi_line: bool) -> (String, usize) {
    let (range, typed) = edit_between(before, after, caret);
    let typed = if multi_line { typed.replace("\r\n", "\n") } else { typed.replace(['\n', '\r'], " ") };
    let (filter, fit) = KINDS.with(|kinds| {
        let kinds = kinds.borrow();
        (kinds.filters.get(kind).cloned(), kinds.fits.get(kind).cloned())
    });
    let allowed: String = typed.chars().filter(|ch| filter.as_ref().is_none_or(|allow| allow(*ch))).collect();
    let kept = before.chars().count() - before[range.clone()].chars().count();
    let typed: String = match max_len {
        0 => allowed,
        max => allowed.chars().take(max.saturating_sub(kept)).collect(),
    };
    if let Some(fit) = fit {
        return fit(before, range, &typed);
    }
    let mut text = before.to_string();
    text.replace_range(range.clone(), &typed);
    (text, range.start + typed.len())
}

/// Wires the `TextFields` global of a component that uses text fields.
#[macro_export]
macro_rules! wire_fields {
    ($app:expr) => {{
        $app.global::<TextFields>().on_admit(|before, after, caret, kind, max_len, multi_line| {
            let (text, caret) = $crate::fields::admit(&before, &after, caret.max(0) as usize, &kind, max_len.max(0) as usize, multi_line);
            TextFit { text: text.into(), caret: caret as i32 }
        });
        $app.global::<TextFields>().on_is_email(|text| $crate::fields::is_email(&text));
        $app.global::<TextFields>().on_is_url(|text| $crate::fields::is_url(&text));
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_the_edit() {
        assert_eq!(edit_between("abc", "abXc", 3), (2..2, "X"));
        assert_eq!(edit_between("abc", "ac", 1), (1..2, ""));
        assert_eq!(edit_between("aaa", "aaaa", 4), (3..3, "a"));
        assert_eq!(edit_between("aaa", "aaaa", 1), (0..0, "a"));
        assert_eq!(edit_between("héllo", "hallo", 2), (1..3, "a"));
        assert_eq!(edit_between("", "x", 1), (0..0, "x"));
        assert_eq!(edit_between("abcd", "aXd", 2), (1..3, "X"));
    }

    #[test]
    fn filters_caps_and_flattens() {
        register_filter("digits", |ch| ch.is_ascii_digit());
        assert_eq!(admit("12", "12a3", 4, "digits", 0, false), ("123".into(), 3));
        assert_eq!(admit("abc", "abcdef", 6, "", 4, false), ("abcd".into(), 4));
        assert_eq!(admit("a", "a\nb", 3, "", 0, false), ("a b".into(), 3));
        assert_eq!(admit("a", "a\r\nb", 4, "", 0, true), ("a\nb".into(), 3));
    }

    #[test]
    fn emails_and_urls() {
        assert!(is_email("ada@example.com"));
        assert!(!is_email("ada@example"));
        assert!(!is_email("ada example.com"));
        assert!(is_url("https://ely.dev/docs?x=1"));
        assert!(is_url("http://localhost.test"));
        assert!(!is_url("ely.dev"));
        assert!(!is_url("https://nodot"));
    }

    #[test]
    fn fits_reshape() {
        register_fit("upper", |base, range, typed| {
            let mut text = base.to_string();
            text.replace_range(range.clone(), &typed.to_uppercase());
            let caret = range.start + typed.len();
            (text, caret)
        });
        assert_eq!(admit("A", "Ab", 2, "upper", 0, false), ("AB".into(), 2));
    }
}
