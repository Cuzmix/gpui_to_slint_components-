//! What option lists cannot do in `.slint`: walk their rows. Ported from ely-core
//! `forms/options.rs` (`step`). The `Choices` global in `ui/components/options.slint` reaches it
//! through a pure callback the host wires with [`wire_options!`](crate::wire_options).

/// The next row from `from` that is not disabled, stepping by `by` and wrapping; `from` when
/// every row is disabled (ely `step`).
pub fn step(disabled: &[bool], from: usize, by: isize) -> usize {
    let count = disabled.len() as isize;
    let mut at = from as isize;
    for _ in 0..count {
        at = (at + by).rem_euclid(count);
        if !disabled[at as usize] {
            return at as usize;
        }
    }
    from
}

/// Rows whose label or note holds `query`, ignoring case; all rows for an empty query (ely
/// `matching`).
pub fn matching<'a, R>(rows: &'a [R], query: &str, label: impl Fn(&R) -> &str, note: impl Fn(&R) -> &str) -> Vec<&'a R> {
    let query = query.trim().to_lowercase();
    let holds = |text: &str| text.to_lowercase().contains(&query);
    rows.iter().filter(|row| holds(label(row)) || (!note(row).is_empty() && holds(note(row)))).collect()
}

/// True when the text ends inside a quoted string (ely tags.rs `in_quotes`).
pub fn in_quotes(text: &str) -> bool {
    let (mut inside, mut escaped) = (false, false);
    for ch in text.chars() {
        match ch {
            _ if escaped => escaped = false,
            '\\' if inside => escaped = true,
            '"' => inside = !inside,
            _ => {}
        }
    }
    inside
}

/// The word a tag field holds: trimmed, a trailing comma dropped (ely `Tags::take`).
pub fn tag_word(text: &str) -> &str {
    text.trim().trim_end_matches(',').trim()
}

/// `selected` with `value` turned on or off, in choice order (ely check.rs `toggled`).
pub fn toggled<V: PartialEq + Clone>(choices: &[V], selected: &[V], value: &V, on: bool) -> Vec<V> {
    choices
        .iter()
        .filter(|candidate| if *candidate == value { on } else { selected.contains(candidate) })
        .cloned()
        .collect()
}

/// Wires the `Choices` global of a component that shows option lists.
#[macro_export]
macro_rules! wire_options {
    ($app:expr) => {{
        use ::slint::Model;
        $app.global::<Choices>().on_step(|rows, from, by| {
            let disabled: Vec<bool> = rows.iter().map(|row| row.disabled).collect();
            if disabled.is_empty() {
                return from;
            }
            $crate::options::step(&disabled, from.max(0) as usize, by as isize) as i32
        });
        $app.global::<Choices>().on_matching(|rows, query, except| {
            let rows: Vec<Choice> = rows.iter().collect();
            let except: Vec<::slint::SharedString> = except.iter().collect();
            let kept: Vec<Choice> = $crate::options::matching(&rows, &query, |row| row.label.as_str(), |row| row.note.as_str())
                .into_iter()
                .filter(|row| !except.contains(&row.value))
                .cloned()
                .collect();
            ::slint::ModelRc::new(::slint::VecModel::from(kept))
        });
        $app.global::<Choices>().on_index_of(|rows, value| rows.iter().position(|row| row.value == value).map_or(-1, |ix| ix as i32));
        $app.global::<Choices>().on_toggled(|choices, selected, value, on| {
            let values: Vec<::slint::SharedString> = choices.iter().map(|row| row.value).collect();
            let selected: Vec<::slint::SharedString> = selected.iter().collect();
            ::slint::ModelRc::new(::slint::VecModel::from($crate::options::toggled(&values, &selected, &value, on)))
        });
        $app.global::<Choices>().on_label_of(|rows, value| {
            rows.iter().find(|row| row.value == value).map_or(value, |row| row.label)
        });
        $app.global::<Choices>().on_with(|values, value| {
            let mut values: Vec<::slint::SharedString> = values.iter().collect();
            if !value.is_empty() && !values.contains(&value) {
                values.push(value);
            }
            ::slint::ModelRc::new(::slint::VecModel::from(values))
        });
        $app.global::<Choices>().on_without(|values, index| {
            let mut values: Vec<::slint::SharedString> = values.iter().collect();
            if index >= 0 && (index as usize) < values.len() {
                values.remove(index as usize);
            }
            ::slint::ModelRc::new(::slint::VecModel::from(values))
        });
        $app.global::<Choices>().on_tag_word(|text| $crate::options::tag_word(&text).into());
        $app.global::<Choices>().on_comma_ends(|text| text.ends_with(',') && !$crate::options::in_quotes(&text));
        $app.global::<Choices>().on_plain(|labels| {
            let rows: Vec<Choice> = labels.iter().map(|label| Choice { value: label.clone(), label, ..Default::default() }).collect();
            ::slint::ModelRc::new(::slint::VecModel::from(rows))
        });
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn toggling_keeps_choice_order() {
        let choices = ["a", "b", "c"];
        assert_eq!(super::toggled(&choices, &["c"], &"a", true), ["a", "c"]);
        assert_eq!(super::toggled(&choices, &["a", "c"], &"a", false), ["c"]);
    }

    use super::{in_quotes, matching, step, tag_word};

    #[test]
    fn matching_ignores_case_and_keeps_order() {
        let rows = [("Berlin", ""), ("Lisbon", ""), ("Oslo", "NO")];
        let labels = |query| matching(&rows, query, |row| row.0, |row| row.1).into_iter().map(|row| row.0).collect::<Vec<_>>();
        assert_eq!(labels("LI"), ["Berlin", "Lisbon"]);
        assert_eq!(labels(""), ["Berlin", "Lisbon", "Oslo"]);
        assert!(labels("xyz").is_empty());
        assert_eq!(labels("no"), ["Oslo"]);
    }

    #[test]
    fn commas_inside_quotes_stay() {
        assert!(in_quotes("\"a,"));
        assert!(!in_quotes("\"a\","));
        assert!(in_quotes("\"a\\\",")); // an escaped quote keeps it open
        assert_eq!(tag_word("  rust , "), "rust");
    }

    #[test]
    fn stepping_wraps_and_skips_disabled_rows() {
        let rows = [false, true, false];
        assert_eq!(step(&rows, 0, 1), 2);
        assert_eq!(step(&rows, 2, 1), 0);
        assert_eq!(step(&rows, 0, -1), 2);
        assert_eq!(step(&[true], 0, 1), 0);
    }
}
