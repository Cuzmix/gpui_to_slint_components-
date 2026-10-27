# Spec: NumberText, CurrencyText, PercentText, FileSizeText, DurationText, PluralText, RelativeTime, DateTimeText

Source: `ely-core/src/typography/formatted.rs`, `typography/format.rs`.

Every value text is a text in the ambient style with tabular figures (G20). English separators: `,` between groups of three, `.` before the fraction. Negative values use the typographic minus U+2212, and a value that rounds to zero has no sign.

| Component | Input | Reads |
|---|---|---|
| NumberText | value, decimals (0) | `1,234.50` |
| CurrencyText | amount, ISO 4217 code | symbol for USD $, EUR €, GBP £, JPY/CNY ¥, KRW ₩, INR ₹, else the code and a space; the currency's minor units (ISO 4217, 2 if unknown): `$19.99`, `¥1,200`, `CHF 5.00` |
| PercentText | ratio, decimals (1), signed | `25.0%`; signed adds `+` to a gain that does not round to zero; ink `success` for a gain, `danger` for a loss, else `fg` |
| FileSizeText | bytes, binary | B, KB, MB, GB, TB, PB by 1000, or KiB… by 1024; one decimal under 10, none above; whole bytes as `999 B`; a rounding that reaches the next unit reads `1.0` of it |
| DurationText | seconds, style | clock `01:02:05` (`23:45` under an hour); compact `1d 1h`, `1h 2m`, `4m 5s`, `5s` |
| PluralText | count, one, other | `1 file`, `12,345 files` |
| RelativeTime | moment | under 45 s `just now`; then minutes, hours, days, months (30 d), years (365 d) rounded half up at the source's thresholds (90 s, 45 min, 90 min, 22 h, 36 h, 26 d, 45 d, 320 d, 548 d); `3 minutes ago`, `in 2 days`. Redraws every 30 s while shown. |
| DateTimeText | moment, strftime pattern (`%b %-d, %Y · %H:%M`), zone (system) | `Oct 23, 2026 · 14:05`; an unknown system zone reads UTC and says so |

A bad DateTimeText pattern panics in the source. The port logs it and shows nothing, the conservative reading that keeps the window up; an unknown named zone does the same.

## Slint
- DurationText, PluralText and RelativeTime are pure Slint (`Figures` global). RelativeTime's words go through `@tr` with plural forms, as the source's i18n keys `time.*` do; translations: G26. It reads `Clock.now` (Unix seconds), which `ely_slint::wire_clock!(app)` sets and moves every 30 s.
- NumberText, CurrencyText, PercentText, FileSizeText and DateTimeText call `TextFormat` (`ely_slint::wire_text_format!`). Unwired: Slint's `to-fixed` without grouping, the code before an amount, bytes as `N B`, and an empty date.
- Values are 32-bit: G25.
