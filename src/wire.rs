//! What `wire_text_format!` hands the `TextFormat` global. Each function takes and returns Slint
//! values, so the macro is only glue.

use slint::{ModelRc, SharedString, VecModel};

use crate::keys::{Platform, keystroke, keystroke_labels};
use crate::text;

fn strings(items: Vec<String>) -> ModelRc<SharedString> {
    ModelRc::new(VecModel::from(items.into_iter().map(SharedString::from).collect::<Vec<_>>()))
}

/// The caps of one keystroke; empty if it does not parse.
pub fn key_labels(source: &str, platform: Platform) -> ModelRc<SharedString> {
    match keystroke(source, platform) {
        Ok(stroke) => strings(keystroke_labels(&stroke, platform)),
        Err(error) => {
            log::error!("kbd: {error}");
            strings(Vec::new())
        }
    }
}

/// The caps of each chord; a chord that does not parse shows its source on one cap.
pub fn key_chords(source: &str, platform: Platform) -> ModelRc<ModelRc<SharedString>> {
    let chords: Vec<_> = source
        .split_whitespace()
        .map(|chord| match keystroke(chord, platform) {
            Ok(stroke) => strings(keystroke_labels(&stroke, platform)),
            Err(error) => {
                log::error!("kbd combo: {error}");
                strings(vec![chord.to_string()])
            }
        })
        .collect();
    ModelRc::new(VecModel::from(chords))
}

/// Every end cut, shortest first.
pub fn end_cuts(text: &str) -> ModelRc<SharedString> {
    strings(text::end_cuts(text))
}

/// Every middle cut, shortest first.
pub fn middle_cuts(text: &str) -> ModelRc<SharedString> {
    strings(text::middle_cuts(text))
}

/// Alternating plain and marked runs, first plain.
pub fn highlight_runs(text: &str, query: &str) -> ModelRc<SharedString> {
    strings(text::highlight_runs(text, &text::matches(text, query)))
}

/// Wires the `TextFormat` global of `app` to this crate. The host's root `.slint` file exports
/// `TextFormat` and `PlatformStyle`, and the generated types are in scope where this is called.
///
/// ```ignore
/// let app = App::new()?;
/// ely_slint::wire_text_format!(app);
/// ```
#[macro_export]
macro_rules! wire_text_format {
    ($app:expr) => {{
        fn platform(style: PlatformStyle) -> $crate::Platform {
            match style {
                PlatformStyle::Mac => $crate::Platform::Mac,
                PlatformStyle::Windows => $crate::Platform::Windows,
                PlatformStyle::Linux => $crate::Platform::Linux,
            }
        }
        let format = $app.global::<TextFormat>();
        format.on_key_labels(|source, style| $crate::wire::key_labels(&source, platform(style)));
        format.on_key_chords(|source, style| $crate::wire::key_chords(&source, platform(style)));
        format.on_end_cuts(|text| $crate::wire::end_cuts(&text));
        format.on_middle_cuts(|text| $crate::wire::middle_cuts(&text));
        format.on_highlight_runs(|text, query| $crate::wire::highlight_runs(&text, &query));
    }};
}
