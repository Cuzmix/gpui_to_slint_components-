//! What `wire_text_format!` hands the `TextFormat` global. Each function takes and returns Slint
//! values, so the macro is only glue.

use slint::{Color, ModelRc, SharedString, StyledText, VecModel};

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

/// Grapheme clusters in `text`.
pub fn grapheme_count(text: &str) -> i32 {
    i32::try_from(text::graphemes(text).len()).unwrap_or(i32::MAX)
}

/// The first `count` grapheme clusters.
pub fn grapheme_prefix(text: &str, count: i32) -> SharedString {
    text::grapheme_prefix(text, count.max(0) as usize).into()
}

fn rgba(color: Color) -> [u8; 4] {
    [color.red(), color.green(), color.blue(), color.alpha()]
}

fn styled(markdown: &str, text: &str) -> StyledText {
    StyledText::from_markdown(markdown).unwrap_or_else(|error| {
        log::error!("styled text: {error}");
        StyledText::from_plain_text(text)
    })
}

/// A ramp from `from` to `to` across the graphemes.
pub fn gradient(text: &str, from: Color, to: Color) -> StyledText {
    styled(&text::gradient_markdown(text, rgba(from), rgba(to)), text)
}

/// The shimmer's band at loop position `t`.
pub fn shimmer(text: &str, dim: Color, lit: Color, t: f32) -> StyledText {
    styled(&text::shimmer_markdown(text, rgba(dim), rgba(lit), t), text)
}

/// AnimatedNumber's text.
pub fn rolling_text(value: f32, decimals: i32, pad: i32) -> SharedString {
    text::rolling_text(f64::from(value), decimals.max(0) as usize, pad.max(0) as usize).into()
}

/// Two texts aligned by place, as (from, to) pairs.
pub fn roll_pairs(from: &str, to: &str) -> Vec<(SharedString, SharedString)> {
    let (from, to) = text::align(from, to);
    from.into_iter().zip(to).map(|(from, to)| (from.into(), to.into())).collect()
}

/// The current moment in Unix seconds, as `ely_slint::format::now` reads it.
pub fn now() -> i32 {
    i32::try_from(format::now().as_second()).unwrap_or(i32::MAX)
}

/// Wires the `TextFormat` global of `app` to this crate. The host's root `.slint` file exports
/// `TextFormat`, `PlatformStyle` and `RollCell`, and the generated types are in scope where this is called.
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
        format.on_grapheme_count(|text| $crate::wire::grapheme_count(&text));
        format.on_grapheme_prefix(|text, count| $crate::wire::grapheme_prefix(&text, count));
        format.on_gradient(|text, from, to| $crate::wire::gradient(&text, from, to));
        format.on_shimmer(|text, dim, lit, t| $crate::wire::shimmer(&text, dim, lit, t));
        format.on_rolling_text(|value, decimals, pad| $crate::wire::rolling_text(value, decimals, pad));
        format.on_roll_cells(|from, to| {
            let cells: Vec<RollCell> = $crate::wire::roll_pairs(&from, &to)
                .into_iter()
                .map(|(from, to)| RollCell { from, to })
                .collect();
            ::slint::ModelRc::new(::slint::VecModel::from(cells))
        });
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
