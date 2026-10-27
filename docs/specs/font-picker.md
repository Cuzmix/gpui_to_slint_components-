# Spec: FontPicker

Source: `ely-core/src/forms/catalogs.rs` (`FontPicker`).

- A Combobox over the system's font families (hidden ones, starting with ".", left out), found once and logged with their count.
- Once a family is chosen, 8 px below the field a sample line, "The quick brown fox jumps over the lazy dog." (`theme.size.sample`), set in that family in lg fg-muted.

## Slint mapping
- `FontPicker inherits VerticalLayout { owner, family <=>, placeholder, label, enabled; family-changed }`, out `open`, `has-focus`, `text`. `family` is in-out (G48); empty means none.
- The families come from `Catalogs.fonts`, filled by `wire_catalogs!` from fontique's system collection, the scan Slint's own text stack uses (G65).
