// LanguageSelect, CountrySelect, CurrencySelect and FontPicker, after ely-core/src/forms/catalogs.rs.
#[macro_use]
mod common;
use common::*;
use i_slint_backend_testing::ElementHandle;
use slint::platform::Key;
use slint::{ComponentHandle, Model};

slint::slint! {
    import { LanguageSelect, CountrySelect, CurrencySelect, FontPicker, Catalogs, Choice, Choices, Toggles, OptionFloat, OptionLayer, TextFields, TextFit } from "ui/index.slint";
    export { Catalogs, Choice, Choices, Toggles, TextFields, TextFit }
    export component Harness inherits Window {
        width: 500px;
        height: 500px;
        in-out property <string> language;
        in-out property <string> country: "CN";
        in-out property <string> currency: "EUR";
        in-out property <string> family;
        in-out property <int> changes;
        out property <string> country-text: country.text;
        out property <string> currency-text: currency.text;
        out property <string> first-label: OptionFloat.rows.length > 0 ? OptionFloat.rows[0].label : "";
        out property <string> first-value: OptionFloat.rows.length > 0 ? OptionFloat.rows[0].value : "";
        public function focus-language() { language.focus(); }
        public function focus-font() { font.focus(); }
        language := LanguageSelect {
            x: 10px; y: 10px; width: 260px;
            selected <=> root.language;
            selection-changed => { root.changes += 1; }
        }
        country := CountrySelect {
            x: 10px; y: 60px; width: 260px;
            selected <=> root.country;
        }
        currency := CurrencySelect {
            x: 10px; y: 110px; width: 260px;
            selected <=> root.currency;
        }
        font := FontPicker {
            x: 10px; y: 160px; width: 260px;
            family <=> root.family;
            family-changed => { root.changes += 1; }
        }
        OptionLayer { }
    }
}

const SAMPLE: &str = "The quick brown fox jumps over the lazy dog.";

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    ely_slint::wire_fields!(h);
    ely_slint::wire_options!(h);
    ely_slint::wire_toggles!(h);
    ely_slint::wire_catalogs!(h);
    show(&h);
    h
}

fn type_in(h: &Harness, text: &str) {
    for ch in text.chars() {
        tap(h.window(), ch.to_string());
    }
    advance(0);
}

fn the_lists_are_filled_and_show_what_is_chosen() {
    let h = harness();
    let catalogs = h.global::<Catalogs>();
    assert_eq!(catalogs.get_currencies().row_count(), 154);
    assert!(catalogs.get_languages().row_count() > 180);
    assert!(catalogs.get_countries().row_count() > 240);
    assert_eq!(h.get_country_text(), "🇨🇳 China");
    assert_eq!(h.get_currency_text(), "Euro");
}

fn a_language_is_found_by_its_english_name() {
    let h = harness();
    h.invoke_focus_language();
    type_in(&h, "japanese");
    assert_eq!((h.get_first_value().as_str(), h.get_first_label().as_str()), ("ja", "日本語"));
    tap(h.window(), Key::Return);
    advance(0);
    assert_eq!(h.get_language(), "ja");
    assert_eq!(h.get_changes(), 1);
}

fn a_chosen_font_sets_the_sample_line() {
    let h = harness();
    assert_eq!(ElementHandle::find_by_accessible_label(&h, SAMPLE).count(), 0, "no sample before a pick");
    let families = ["Alpha Sans", "Beta Serif"].map(|family| Choice { value: family.into(), label: family.into(), ..Default::default() });
    h.global::<Catalogs>().set_fonts(slint::ModelRc::new(slint::VecModel::from(families.to_vec())));
    h.invoke_focus_font();
    type_in(&h, "beta");
    tap(h.window(), Key::Return);
    advance(0);
    assert_eq!(h.get_family(), "Beta Serif");
    assert_eq!(h.get_changes(), 1);
    assert_eq!(ElementHandle::find_by_accessible_label(&h, SAMPLE).count(), 1);
}

cases!(catalog_fields: the_lists_are_filled_and_show_what_is_chosen, a_language_is_found_by_its_english_name, a_chosen_font_sets_the_sample_line);
