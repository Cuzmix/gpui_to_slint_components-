# Ely components for Slint

A port of [Ely-GPUI-Components](https://github.com/ELY-Open-Source/Ely-GPUI-Components) to [Slint](https://slint.dev) 1.18.1. Every component is built from Slint's builtin elements only, with no widget library and no GPUI. Behavior follows the GPUI source value for value: sizes, colors, timings, keyboard and mouse rules. Where Slint cannot match it, the difference is written down in [`PORTING_GAPS.md`](PORTING_GAPS.md).

- [`INVENTORY.md`](INVENTORY.md): every GPUI component, whether it needs Rust, and the port order.
- [`PORTING_STATUS.md`](PORTING_STATUS.md): the checklist for each ported component.
- [`docs/specs/`](docs/specs): the spec each port was built from, with every value taken from the source.

## Use from Slint alone

Point your Slint compiler at `ui/` and import from `ui/index.slint`:

```slint
import { Theme, ThemeFader, Mode, Button, ButtonVariant, IconButton, Icons } from "ely/index.slint";

export component App inherits Window {
    background: Theme.colors.bg;
    default-font-family: Theme.font-family;

    // One per window: runs the 280 ms light/dark cross-fade.
    ThemeFader { }

    HorizontalLayout {
        spacing: 12px;
        alignment: start;
        Button {
            text: "Save";
            variant: ButtonVariant.primary;
            icon: Icons.check;
            clicked => { debug("saved"); }
        }
        IconButton {
            icon: Icons.settings;
            tooltip: "Settings";
        }
        Button {
            text: Theme.is-dark ? "Light" : "Dark";
            clicked => { Theme.mode = Theme.is-dark ? Mode.light : Mode.dark; }
        }
    }
}
```

With `slint-build`, add the folder as a library path:

```rust
// build.rs
let config = slint_build::CompilerConfiguration::new()
    .with_library_paths([("ely".into(), "path/to/ely-slint/ui".into())].into());
slint_build::compile_with_config("ui/app.slint", config).unwrap();
```

and import with `import { Button } from "@ely/index.slint";`.

## Rust-assisted components

Some GPUI behavior needs logic `.slint` cannot express. The `ely-slint` crate computes it and hands plain values or models to the components; it never owns UI state.

| Component | What the host wires up | Rust helper |
|---|---|---|
| Button (shortcut hint) | `shortcut: string`, already spelled for the platform | `ely_slint::shortcut_text("secondary-s", Platform::current())` gives `⌘S` or `Ctrl+S` |
| FocusTrap | `FocusSteps.step(forward)`, once per window (export `FocusSteps` from your root file) | `ely_slint::focus::step(window, forward)` |
| Picture (network or slow sources) | `loading` while it fetches, then `source` | none: use your own loader |
| Kbd, KbdCombo, Highlight (and later Ellipsis, MiddleEllipsis) | `TextFormat`'s pure callbacks, once per window (export `TextFormat` and `PlatformStyle` from your root file) | `ely_slint::wire_text_format!(app)` |
| IntersectionObserver | `visible-x`/`-y`/`-width`/`-height`, the area that counts as on screen | none |

```rust
use ely_slint::{Platform, shortcut_text};

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let app = App::new()?;
    app.set_save_shortcut(shortcut_text("secondary-s", Platform::current()).unwrap().into());
    app.run()
}
```

Every component that needs Rust lists its globals, callbacks and models in its row of `PORTING_STATUS.md` and in its spec.

## Theme

`Theme` is one global holding every design token: `Theme.colors` (the palette on screen), `Theme.mode` (light or dark), `high-contrast`, `color-blind-safe`, `density`, `radius-scale`, `font-scale`, `reduced-motion`, and the measures (`Theme.control-height(ControlSize.md)`, `Theme.radius(Radius.md)`, `Theme.text-size(TextSize.base)` …). Change `Theme.mode` to switch modes; with a `ThemeFader` in the window the colors cross-fade as in GPUI, and `Theme.set-mode-now(m)` switches at once.

## Gallery

```sh
cargo run --bin gallery
slint-viewer ui/gallery.slint
```

The gallery mirrors the GPUI story app page by page as chapters land.

## Checks

`scripts/check.sh` runs what CI runs: no widget-library import anywhere, `slint-lsp format` clean, `cargo build` and `cargo test`, no GPUI crate in `cargo tree`, and `slint-viewer` loading every component without a warning.

Tests use `i-slint-backend-testing` and need one thread (set in `.cargo/config.toml`), plus `SLINT_EMIT_DEBUG_INFO=1` for element queries.

## Licenses

MIT, as the source. Icons: Lucide (ISC), `ui/icons/LICENSE`. Fonts: Inter and JetBrains Mono (OFL), `ui/fonts/`.
