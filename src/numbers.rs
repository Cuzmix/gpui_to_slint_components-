//! What number fields cannot do in `.slint`: read typed text as a number and settle it. Ported
//! from ely-core `forms/number/value.rs`. The `Numbers` global in
//! `ui/components/number-input.slint` reaches it through pure callbacks the host wires with
//! [`wire_numbers!`](crate::wire_numbers).

use crate::format::{self, Separators};

/// What a number field lets through as typed (ely `NumberInput`'s filter).
pub fn typable(ch: char) -> bool {
    ch.is_ascii_digit() || matches!(ch, '.' | ',' | '-' | format::MINUS)
}

/// Reads a typed number: groups and the typographic minus are allowed (ely `parse`).
pub fn parse(text: &str) -> Option<f64> {
    let plain: String = text
        .chars()
        .filter(|ch| *ch != ',')
        .map(|ch| if ch == format::MINUS { '-' } else { ch })
        .collect();
    plain.trim().parse::<f64>().ok().filter(|value| value.is_finite())
}

/// Rounds to `precision` places, then clamps to the rounded numbers in `min..=max` (ely
/// `settle`). An empty range settles on its low end, where gpui panics.
pub fn settle(value: f64, min: f64, max: f64, precision: usize) -> f64 {
    let scale = 10f64.powi(precision as i32);
    let (low, high) = (on_grid(min, scale, f64::ceil), on_grid(max, scale, f64::floor));
    if low > high {
        log::error!("number input: no {precision}-place number lies in {min}..={max}");
        return low;
    }
    ((value * scale).round() / scale).clamp(low, high)
}

/// `bound` on the `1 / scale` grid: kept if float error alone put it off, else moved `inward`.
fn on_grid(bound: f64, scale: f64, inward: fn(f64) -> f64) -> f64 {
    let scaled = bound * scale;
    let near = scaled.round();
    if (scaled - near).abs() <= 4.0 * f64::EPSILON * near.abs().max(1.0) { near / scale } else { inward(scaled) / scale }
}

/// The number as a field shows it: English separators, `precision` places.
pub fn text(value: f64, precision: usize) -> String {
    format::number(value, precision, Separators::EN)
}

/// Slint's `float` is single precision: a value read back through it is first put on the
/// shortest decimal that rounds to it, so 0.1 stays 0.1.
pub fn widen(value: f32) -> f64 {
    value.to_string().parse().unwrap_or(f64::from(value))
}

/// Wires the `Numbers` global of a component that uses number fields, and the `number` kind of
/// text field it types in (call `wire_fields!` too).
#[macro_export]
macro_rules! wire_numbers {
    ($app:expr) => {{
        $crate::fields::register_filter("number", $crate::numbers::typable);
        let numbers = $app.global::<Numbers>();
        numbers.on_parse(|text| match $crate::numbers::parse(&text) {
            Some(value) => ParsedNumber { ok: true, value: value as f32 },
            None => ParsedNumber { ok: false, value: 0.0 },
        });
        numbers.on_settle(|value, min, max, precision| {
            let w = $crate::numbers::widen;
            $crate::numbers::settle(w(value), w(min), w(max), precision.max(0) as usize) as f32
        });
        numbers.on_text(|value, precision| $crate::numbers::text($crate::numbers::widen(value), precision.max(0) as usize).into());
        numbers.on_currency_symbol(|code| $crate::format::currency_parts(&code).0.into());
        numbers.on_currency_places(|code| $crate::format::currency_parts(&code).1 as i32);
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_groups_and_the_minus() {
        assert_eq!(parse("1,234.5"), Some(1234.5));
        assert_eq!(parse("\u{2212}3"), Some(-3.0));
        assert_eq!(parse(" 7 "), Some(7.0));
        assert_eq!(parse("1.2.3"), None);
        assert_eq!(parse(""), None);
    }

    #[test]
    fn settles_on_the_grid_inside_the_range() {
        assert_eq!(settle(2.345, 0.0, 10.0, 2), 2.35);
        assert_eq!(settle(12.0, 0.0, 10.0, 0), 10.0);
        assert_eq!(settle(0.0, 0.05, 1.0, 1), 0.1);
        assert_eq!(settle(0.3, 0.1 + 0.2, 1.0, 1), 0.3);
    }

    #[test]
    fn widening_keeps_short_decimals() {
        assert_eq!(widen(0.1), 0.1);
        assert_eq!(text(widen(1234.5), 2), "1,234.50");
    }
}
