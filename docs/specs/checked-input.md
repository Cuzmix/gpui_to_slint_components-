# Spec: EmailInput, UrlInput

Source: `ely-core/src/forms/checked.rs`.

- A column 4 px apart: an Input with a sm prefix icon in fg-subtle (at-sign, link), invalid once it holds text that misses its check and focus has left; then, on a miss, a note in xs danger: "Enter an address like name@example.com", "Enter a link that starts with https://".
- Checks: email `^[^\s@]+@[^\s@]+\.[^\s@]+$`; link `^https?://[^\s/?#]+\.[^\s/?#]+\S*$`.

## Slint mapping
- `EmailInput` / `UrlInput { text, placeholder, label, size, enabled }`; out `miss`. The checks run in Rust (`TextFields.is-email`, `is-url`, wired by `wire_fields!`).
