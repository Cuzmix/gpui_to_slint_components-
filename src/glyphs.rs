//! What IconPicker and EmojiPicker cannot do in `.slint`: find glyphs, lay them in rows and walk
//! them with the arrows. Ported from ely-core `forms/picks.rs` (`rows`, `row_of`, `stepped`, the
//! finder's query) and `forms/glyphs.rs` (`emoji_found`, the group titles, the icon filter).

use std::ops::Range;

use emojis::Group;

/// Glyphs across an emoji or icon grid.
pub const COLUMNS: usize = 8;

/// A row of a glyph grid: a group's title, or a run of glyphs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Row {
    Title(String),
    Glyphs(Range<usize>),
}

/// Rows for glyph groups given as (title, count), laid in order, `columns` across.
pub fn rows(groups: impl IntoIterator<Item = (Option<String>, usize)>, columns: usize) -> Vec<Row> {
    let (mut out, mut start) = (Vec::new(), 0);
    for (title, count) in groups.into_iter().filter(|(_, count)| *count > 0) {
        out.extend(title.map(Row::Title));
        let end = start + count;
        while start < end {
            let stop = (start + columns).min(end);
            out.push(Row::Glyphs(start..stop));
            start = stop;
        }
    }
    out
}

/// The row holding glyph `ix`.
pub fn row_of(rows: &[Row], ix: usize) -> Option<usize> {
    rows.iter().position(|row| matches!(row, Row::Glyphs(range) if range.contains(&ix)))
}

/// The glyph an arrow key moves to from `at`: same column in the next row, or the last one there.
pub fn stepped(rows: &[Row], at: usize, key: &str) -> Option<usize> {
    let row = row_of(rows, at)?;
    let Row::Glyphs(here) = &rows[row] else {
        unreachable!("row_of finds glyph rows");
    };
    let column = at - here.start;
    let land = |row: &Row| match row {
        Row::Glyphs(range) => Some((range.start + column).min(range.end - 1)),
        Row::Title(_) => None,
    };
    let last = rows.iter().rev().find_map(|row| match row {
        Row::Glyphs(range) => Some(range.end - 1),
        Row::Title(_) => None,
    })?;
    match key {
        "left" => at.checked_sub(1),
        "right" => (at < last).then_some(at + 1),
        "up" => rows[..row].iter().rev().find_map(land),
        "down" => rows[row + 1..].iter().find_map(land),
        _ => None,
    }
}

/// What a search field holds, as the grids match it: trimmed and lowercase.
pub fn query(text: &str) -> String {
    text.trim().to_lowercase()
}

/// What a grid shows: its rows, and each glyph's name and what draws it.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Layout {
    pub rows: Vec<Row>,
    pub names: Vec<String>,
    /// The emoji, for an emoji grid.
    pub glyphs: Vec<String>,
    /// Where each icon sits among all of them, for an icon grid.
    pub at: Vec<usize>,
}

/// The icons among `names` whose name, with spaces for dashes, holds the search text.
pub fn icon_layout<S: AsRef<str>>(names: &[S], text: &str) -> Layout {
    let query = query(text);
    let at: Vec<usize> = (0..names.len()).filter(|&ix| names[ix].as_ref().replace('-', " ").contains(&query)).collect();
    Layout { rows: rows([(None, at.len())], COLUMNS), names: at.iter().map(|&ix| names[ix].as_ref().to_string()).collect(), glyphs: Vec::new(), at }
}

/// Emoji whose name or shortcode holds `query`, a lowercase string; all for an empty one.
pub fn emoji_found(query: &str) -> Vec<&'static emojis::Emoji> {
    emojis::iter()
        .filter(|emoji| query.is_empty() || emoji.name().to_lowercase().contains(query) || emoji.shortcodes().any(|code| code.contains(query)))
        .collect()
}

fn title(group: Group) -> &'static str {
    match group {
        Group::SmileysAndEmotion => "Smileys & Emotion",
        Group::PeopleAndBody => "People & Body",
        Group::AnimalsAndNature => "Animals & Nature",
        Group::FoodAndDrink => "Food & Drink",
        Group::TravelAndPlaces => "Travel & Places",
        Group::Activities => "Activities",
        Group::Objects => "Objects",
        Group::Symbols => "Symbols",
        Group::Flags => "Flags",
    }
}

/// The emoji matching the search text: under their Unicode groups while it is empty.
pub fn emoji_layout(text: &str) -> Layout {
    let query = query(text);
    let found = emoji_found(&query);
    let rows = if query.is_empty() {
        rows(Group::iter().map(|group| (Some(title(group).to_string()), group.emojis().count())), COLUMNS)
    } else {
        rows([(None, found.len())], COLUMNS)
    };
    Layout {
        rows,
        names: found.iter().map(|emoji| emoji.name().to_string()).collect(),
        glyphs: found.iter().map(|emoji| emoji.as_str().to_string()).collect(),
        at: Vec::new(),
    }
}

/// Wires the `Glyphs` global, which every IconPicker and EmojiPicker asks. The host's root
/// `.slint` file exports `Glyphs`, `GlyphLayout`, `GlyphRow` and `IconEntry`.
///
/// ```ignore
/// ely_slint::wire_glyphs!(app);
/// ```
#[macro_export]
macro_rules! wire_glyphs {
    ($app:expr) => {{
        use ::slint::{ComponentHandle, Model};
        fn strings(items: Vec<String>) -> ::slint::ModelRc<::slint::SharedString> {
            ::slint::ModelRc::new(::slint::VecModel::from(items.into_iter().map(::slint::SharedString::from).collect::<Vec<_>>()))
        }
        fn rows(model: &::slint::ModelRc<GlyphRow>) -> Vec<$crate::glyphs::Row> {
            model
                .iter()
                .map(|row| match row.title.as_str() {
                    "" => $crate::glyphs::Row::Glyphs(row.start.max(0) as usize..row.end.max(0) as usize),
                    title => $crate::glyphs::Row::Title(title.to_string()),
                })
                .collect()
        }
        fn layout(layout: $crate::glyphs::Layout) -> GlyphLayout {
            let rows: Vec<GlyphRow> = layout
                .rows
                .into_iter()
                .map(|row| match row {
                    $crate::glyphs::Row::Title(title) => GlyphRow { title: title.into(), start: 0, end: 0 },
                    $crate::glyphs::Row::Glyphs(range) => GlyphRow { title: Default::default(), start: range.start as i32, end: range.end as i32 },
                })
                .collect();
            let at: Vec<i32> = layout.at.into_iter().map(|ix| ix as i32).collect();
            GlyphLayout {
                rows: ::slint::ModelRc::new(::slint::VecModel::from(rows)),
                names: strings(layout.names),
                glyphs: strings(layout.glyphs),
                at: ::slint::ModelRc::new(::slint::VecModel::from(at)),
            }
        }
        let glyphs = $app.global::<Glyphs>();
        glyphs.on_icon_layout(|icons, text| {
            let names: Vec<::slint::SharedString> = icons.iter().map(|icon: IconEntry| icon.name).collect();
            layout($crate::glyphs::icon_layout(&names, &text))
        });
        glyphs.on_emoji_layout(|text| layout($crate::glyphs::emoji_layout(&text)));
        glyphs.on_step(|given, at, key| {
            let given = rows(&given);
            $crate::glyphs::stepped(&given, at.max(0) as usize, &key).map_or(-1, |to| to as i32)
        });
        glyphs.on_row_of(|given, at| $crate::glyphs::row_of(&rows(&given), at.max(0) as usize).map_or(-1, |row| row as i32));
        glyphs.on_index_of(|items, item| items.iter().position(|it| it == item).map_or(-1, |ix| ix as i32));
        glyphs.on_picked(|what, name| $crate::log::info!("{what}: {name}"));
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn groups_break_into_titled_rows_of_eight() {
        let laid = rows([(Some("A".into()), 10), (None, 0), (Some("B".into()), 3)], 8);
        assert_eq!(laid, [Row::Title("A".into()), Row::Glyphs(0..8), Row::Glyphs(8..10), Row::Title("B".into()), Row::Glyphs(10..13)]);
    }

    #[test]
    fn arrows_keep_the_column_and_cross_titles() {
        let laid = rows([(Some("A".into()), 10), (Some("B".into()), 3)], 8);
        assert_eq!(stepped(&laid, 6, "down"), Some(9));
        assert_eq!(stepped(&laid, 9, "down"), Some(11));
        assert_eq!(stepped(&laid, 11, "up"), Some(9));
        assert_eq!(stepped(&laid, 1, "up"), None);
        assert_eq!(stepped(&laid, 12, "right"), None);
        assert_eq!(stepped(&laid, 0, "left"), None);
        assert_eq!(stepped(&laid, 8, "left"), Some(7));
    }

    #[test]
    fn emoji_are_found_whatever_the_case_of_their_names() {
        let glyphs = |query| emoji_found(query).into_iter().map(|emoji| emoji.as_str()).collect::<Vec<_>>();
        assert!(glyphs("united states").contains(&"🇺🇸"));
        assert!(glyphs("sparkles").contains(&"✨"));
        assert_eq!(glyphs("").len(), emojis::iter().count());
    }

    #[test]
    fn icons_match_with_spaces_for_dashes_and_emoji_group_while_unsearched() {
        let names = ["arrow-up", "arrow-down", "star"];
        let found = icon_layout(&names, "  Arrow D ");
        assert_eq!((found.at.as_slice(), found.names.as_slice()), (&[1][..], &["arrow-down".to_string()][..]));
        assert_eq!(icon_layout(&names, "").rows, [Row::Glyphs(0..3)]);
        let all = emoji_layout("");
        assert_eq!(all.rows[0], Row::Title("Smileys & Emotion".into()));
        assert_eq!(all.glyphs.len(), emojis::iter().count());
        assert!(emoji_layout("SPARKLES").rows.iter().all(|row| matches!(row, Row::Glyphs(_))));
    }
}
