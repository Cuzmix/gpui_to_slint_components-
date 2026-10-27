# Spec: LanguageSelect, CountrySelect, CurrencySelect

Source: `ely-core/src/forms/catalogs.rs` (`Combobox::languages`, `Combobox::countries`, `Combobox::currencies`, `flag`, `titled`), `ely-core/src/forms/languages.rs`.

- Comboboxes over fixed lists, sorted by label (case folded):
  - Languages: ISO 639-1, each under its own name with the first letter capitalized ("Français"), noting the English name and code ("French · fr"); a language without an own name shows its English name and notes its code. Typing either name or the code finds it.
  - Countries: ISO 3166-1 countries and territories as "🇨🇳 China", noting the alpha-2 code.
  - Currencies: ISO 4217 currencies in use (no funds, metals, special or superseded codes; 154), under their names, noting the code.
- Everything else is Combobox's: filtering, keys, the floating list.

## Slint mapping
- Each `inherits Combobox`, setting its `owner` ("language", "country", "currency") and `choices` from the `Catalogs` global, which `wire_catalogs!` fills once from `src/catalogs.rs`. gpui builds them as Combobox constructors.
- They need what Combobox needs: `wire_fields!`, `wire_options!`, `wire_toggles!` and an `OptionLayer` (G54).
