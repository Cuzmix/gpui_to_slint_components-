//! What `wire_text_format!` hands the `TextFormat` global. Each function takes and returns Slint
//! values, so the macro is only glue.

use slint::{ModelRc, SharedString, VecModel};

use crate::format::{self, Separators};
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

/// Grouped digits at fixed precision.
pub fn number(value: f32, decimals: i32) -> SharedString {
    format::number(f64::from(value), decimals.max(0) as usize, Separators::EN).into()
}

/// An amount in a currency.
pub fn currency(amount: f32, code: &str) -> SharedString {
    format::currency(f64::from(amount), code).into()
}

/// A ratio as percent.
pub fn percent(ratio: f32, decimals: i32, signed: bool) -> SharedString {
    format::percent(f64::from(ratio), decimals.max(0) as usize, signed).into()
}

/// Bytes in decimal or binary units; negative counts read as none.
pub fn file_size(bytes: f32, binary: bool) -> SharedString {
    format::file_size(bytes.max(0.0) as u64, binary).into()
}

/// Unix seconds in a pattern and zone; empty, logged, if either does not read.
pub fn datetime(at: i32, pattern: &str, zone: &str) -> SharedString {
    let zone = if zone.is_empty() {
        format::system_zone("DateTimeText")
    } else {
        match jiff::tz::TimeZone::get(zone) {
            Ok(zone) => zone,
            Err(error) => {
                log::error!("DateTimeText zone {zone:?}: {error}");
                return SharedString::new();
            }
        }
    };
    let at = match jiff::Timestamp::from_second(i64::from(at)) {
        Ok(at) => at,
        Err(error) => {
            log::error!("DateTimeText moment {at}: {error}");
            return SharedString::new();
        }
    };
    match format::datetime(at, &zone, pattern) {
        Ok(text) => text.into(),
        Err(error) => {
            log::error!("DateTimeText pattern {pattern:?}: {error}");
            SharedString::new()
        }
    }
}

/// The emoji a shortcode names; empty if unknown.
pub fn emoji(shortcode: &str) -> SharedString {
    text::emoji(shortcode).unwrap_or_default().into()
}

/// The current moment in Unix seconds, as `ely_slint::format::now` reads it.
pub fn now() -> i32 {
    i32::try_from(format::now().as_second()).unwrap_or(i32::MAX)
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
        format.on_number(|value, decimals| $crate::wire::number(value, decimals));
        format.on_currency(|amount, code| $crate::wire::currency(amount, &code));
        format.on_percent(|ratio, decimals, signed| $crate::wire::percent(ratio, decimals, signed));
        format.on_file_size(|bytes, binary| $crate::wire::file_size(bytes, binary));
        format.on_datetime(|at, pattern, zone| $crate::wire::datetime(at, &pattern, &zone));
        format.on_emoji(|shortcode| $crate::wire::emoji(&shortcode));
    }};
}

/// Sets the `Clock` global of `app` to now and moves it on every 30 s, as gpui redraws a
/// relative time. Keep the returned timer: dropping it stops the clock.
///
/// ```ignore
/// let _clock = ely_slint::wire_clock!(app);
/// ```
#[macro_export]
macro_rules! wire_clock {
    ($app:expr) => {{
        $app.global::<Clock>().set_now($crate::wire::now());
        let weak = $app.as_weak();
        let timer = ::slint::Timer::default();
        timer.start(::slint::TimerMode::Repeated, std::time::Duration::from_secs(30), move || {
            if let Some(app) = weak.upgrade() {
                app.global::<Clock>().set_now($crate::wire::now());
            }
        });
        timer
    }};
}
