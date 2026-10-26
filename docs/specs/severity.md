# Spec: Severity

Source: `ely-core/src/primitives/severity.rs`.

- Four levels: info, success, warning, danger.
- `color`: `info`, `success`, `warning`, `danger` from the palette on screen.
- `subtle`: `info_subtle`, `success_subtle`, `warning_subtle`, `danger_subtle`.
- `icon`: info → `info`, success → `circle-check`, warning → `triangle-alert`, danger → `circle-alert`.

Slint: `enum Severity` and `global Severities { color(s), subtle(s), icon(s) }`, pure functions reading `Theme.colors`.
