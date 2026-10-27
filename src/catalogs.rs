//! The lists the regional selects and FontPicker show: ISO 639-1 languages, ISO 3166-1
//! countries, ISO 4217 currencies, and the system's font families. Ported from ely-core
//! `forms/catalogs.rs` (`flag`, `titled`, `languages`, `currencies`, `Combobox::countries`).

use iso_currency::Currency;
use isocountry::CountryCode;

use crate::languages::LANGUAGES;

/// A row of a list: what a pick reports, what it shows, and its quiet note.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub value: String,
    pub label: String,
    pub note: String,
}

impl Entry {
    fn new(value: impl Into<String>, label: impl Into<String>, note: impl Into<String>) -> Self {
        Self { value: value.into(), label: label.into(), note: note.into() }
    }
}

/// The flag emoji of a two-letter region code; empty for anything else.
pub fn flag(region: &str) -> String {
    if region.len() != 2 || !region.bytes().all(|letter| letter.is_ascii_uppercase()) {
        log::error!("no flag for region code {region:?}");
        return String::new();
    }
    region.chars().filter_map(|letter| char::from_u32(0x1F1E6 + (letter as u32 - 'A' as u32))).collect()
}

/// `text` with its first letter capitalized, as list items read.
pub fn titled(text: &str) -> String {
    let mut chars = text.chars();
    chars.next().map_or_else(String::new, |first| first.to_uppercase().chain(chars).collect())
}

fn by_label(mut entries: Vec<Entry>) -> Vec<Entry> {
    entries.sort_by_key(|entry| entry.label.to_lowercase());
    entries
}

/// ISO 639-1 languages under their own names, with the English name and code beside.
pub fn languages() -> Vec<Entry> {
    by_label(
        LANGUAGES
            .iter()
            .map(|&(code, english, own)| match own {
                Some(own) => Entry::new(code, titled(own), format!("{english} · {code}")),
                None => Entry::new(code, english, code),
            })
            .collect(),
    )
}

/// ISO 3166-1 countries and territories, flagged, with their codes.
pub fn countries() -> Vec<Entry> {
    let mut named: Vec<(&str, &str)> = CountryCode::iter().map(|country| (country.name(), country.alpha2())).collect();
    named.sort_by_key(|(name, _)| name.to_lowercase());
    named.into_iter().map(|(name, code)| Entry::new(code, format!("{} {name}", flag(code)), code)).collect()
}

/// ISO 4217 currencies in use, without funds, metals or retired codes.
pub fn currencies() -> Vec<Entry> {
    by_label(
        (0..1000)
            .filter_map(Currency::from_numeric)
            .filter(|currency| !currency.is_fund() && !currency.is_special() && currency.is_superseded().is_none())
            .map(|currency| Entry::new(currency.code(), currency.name().to_string(), currency.code()))
            .collect(),
    )
}

/// The font families the system offers, as Slint's text stack finds them, without hidden ones.
pub fn font_families() -> Vec<Entry> {
    let mut collection = fontique::Collection::new(fontique::CollectionOptions { shared: false, system_fonts: true });
    let mut found: Vec<String> = collection.family_names().filter(|family| !family.starts_with('.')).map(str::to_string).collect();
    found.sort();
    found.dedup();
    log::info!("font picker: {} families", found.len());
    found.into_iter().map(|family| Entry::new(family.clone(), family, "")).collect()
}

/// Fills the `Catalogs` global with languages, countries, currencies and font families. The
/// host's root `.slint` file exports `Catalogs` and `Choice`.
///
/// ```ignore
/// ely_slint::wire_catalogs!(app);
/// ```
#[macro_export]
macro_rules! wire_catalogs {
    ($app:expr) => {{
        use ::slint::ComponentHandle;
        fn model(entries: Vec<$crate::catalogs::Entry>) -> ::slint::ModelRc<Choice> {
            let rows: Vec<Choice> = entries
                .into_iter()
                .map(|entry| Choice { value: entry.value.into(), label: entry.label.into(), note: entry.note.into(), ..Default::default() })
                .collect();
            ::slint::ModelRc::new(::slint::VecModel::from(rows))
        }
        let catalogs = $app.global::<Catalogs>();
        catalogs.set_languages(model($crate::catalogs::languages()));
        catalogs.set_countries(model($crate::catalogs::countries()));
        catalogs.set_currencies(model($crate::catalogs::currencies()));
        catalogs.set_fonts(model($crate::catalogs::font_families()));
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    fn matching<'a>(entries: &'a [Entry], query: &str) -> Vec<&'a str> {
        crate::options::matching(entries, query, |entry| entry.label.as_str(), |entry| entry.note.as_str()).into_iter().map(|entry| entry.value.as_str()).collect()
    }

    #[test]
    fn languages_answer_to_their_code_and_english_name() {
        let all = languages();
        assert!(matching(&all, "zh").contains(&"zh"));
        assert!(matching(&all, "chinese").contains(&"zh"));
        assert!(matching(&all, "中文").contains(&"zh"));
        assert!(matching(&all, "aymara").contains(&"ay"));
    }

    #[test]
    fn currencies_in_use_are_listed_once() {
        let codes: Vec<String> = currencies().into_iter().map(|entry| entry.value).collect();
        assert_eq!(codes.len(), 154);
        assert!(codes.iter().any(|code| code == "USD") && codes.iter().any(|code| code == "EUR"));
        assert!(!codes.iter().any(|code| code == "XAU" || code == "USN"), "no metals or funds");
    }

    #[test]
    fn countries_are_flagged_and_sorted() {
        let all = countries();
        let china = all.iter().find(|entry| entry.value == "CN").unwrap();
        assert!(china.label.starts_with("🇨🇳 "), "{}", china.label);
        assert!(all.windows(2).all(|pair| pair[0].label.split_once(' ').unwrap().1.to_lowercase() <= pair[1].label.split_once(' ').unwrap().1.to_lowercase()));
    }

    #[test]
    fn flags_come_from_region_letters() {
        assert_eq!(flag("CN"), "🇨🇳");
        assert_eq!(flag("US"), "🇺🇸");
        assert_eq!(flag("us"), "");
    }

    #[test]
    fn titles_capitalize_the_first_letter_only() {
        assert_eq!(titled("français"), "Français");
        assert_eq!(titled("русский"), "Русский");
        assert_eq!(titled("中文"), "中文");
        assert_eq!(titled(""), "");
    }
}
