//! What the color components cannot do in `.slint`: read typed hex, and grow, move or cut a
//! gradient's list of stops. Ported from ely-core `forms/color/mod.rs` (`parse_hex`, `hex`) and
//! `forms/color/gradient.rs` (`color_at`, `moved`, `added`). Colors cross into Slint as
//! `slint::Color`, eight bits a channel (gap G62).

use slint::Color;

/// A color as four channels from 0 to 1.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Rgba {
    pub fn of(color: Color) -> Self {
        let byte = |value: u8| f32::from(value) / 255.0;
        Self { r: byte(color.red()), g: byte(color.green()), b: byte(color.blue()), a: byte(color.alpha()) }
    }

    pub fn color(self) -> Color {
        let byte = |value: f32| (value.clamp(0.0, 1.0) * 255.0).round() as u8;
        Color::from_argb_u8(byte(self.a), byte(self.r), byte(self.g), byte(self.b))
    }

    /// The blend a share `t` of the way to `to`, per channel (ely's `Mix`).
    pub fn mix(self, to: Self, t: f32) -> Self {
        if t <= 0.0 {
            return self;
        }
        if t >= 1.0 {
            return to;
        }
        let lerp = |x: f32, y: f32| x + (y - x) * t;
        Self { r: lerp(self.r, to.r), g: lerp(self.g, to.g), b: lerp(self.b, to.b), a: lerp(self.a, to.a) }
    }
}

/// Reads `#rgb`, `#rgba`, `#rrggbb` or `#rrggbbaa`, with or without the hash.
pub fn parse_hex(text: &str) -> Result<Rgba, String> {
    let digits = text.trim().trim_start_matches('#');
    if !digits.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(format!("{text:?} is not a hex color"));
    }
    let doubled = || digits.chars().flat_map(|ch| [ch, ch]);
    let long: String = match digits.len() {
        3 => doubled().chain("ff".chars()).collect(),
        4 => doubled().collect(),
        6 => format!("{digits}ff"),
        8 => digits.to_string(),
        _ => return Err(format!("{text:?} is not a hex color")),
    };
    let value = u32::from_str_radix(&long, 16).map_err(|_| format!("{text:?} is not a hex color"))?;
    let channel = |shift: u32| ((value >> shift) & 0xff) as f32 / 255.0;
    Ok(Rgba { r: channel(24), g: channel(16), b: channel(8), a: channel(0) })
}

/// `#rrggbb`, with alpha appended when it is not opaque.
pub fn hex(rgba: Rgba) -> String {
    let byte = |value: f32| (value.clamp(0.0, 1.0) * 255.0).round() as u8;
    let base = format!("#{:02x}{:02x}{:02x}", byte(rgba.r), byte(rgba.g), byte(rgba.b));
    if byte(rgba.a) == 255 { base } else { format!("{base}{:02x}", byte(rgba.a)) }
}

/// A color at a place along a gradient, from 0 to 1.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Stop {
    pub at: f32,
    pub color: Rgba,
}

/// The color at `t` along stops sorted by place; transparent black with no stops.
pub fn color_at(stops: &[Stop], t: f32) -> Rgba {
    let (Some(first), Some(last)) = (stops.first(), stops.last()) else {
        log::error!("gradient editor: no stops to read a color from");
        return Rgba { r: 0.0, g: 0.0, b: 0.0, a: 0.0 };
    };
    if t <= first.at {
        return first.color;
    }
    for pair in stops.windows(2) {
        let (from, to) = (pair[0], pair[1]);
        if t <= to.at {
            let span = (to.at - from.at).max(f32::EPSILON);
            return from.color.mix(to.color, (t - from.at) / span);
        }
    }
    last.color
}

/// `stops` with `ix` moved to `t`, still sorted, and where `ix` landed; unchanged when `ix` is
/// not a stop.
pub fn moved(stops: &[Stop], ix: usize, t: f32) -> (Vec<Stop>, usize) {
    if ix >= stops.len() {
        log::error!("gradient editor: no stop {ix} to move");
        return (stops.to_vec(), ix.min(stops.len().saturating_sub(1)));
    }
    let mut next = stops.to_vec();
    let mut stop = next.remove(ix);
    stop.at = t.clamp(0.0, 1.0);
    let landed = next.partition_point(|other| other.at <= stop.at);
    next.insert(landed, stop);
    (next, landed)
}

/// `stops` with a new stop at `t`, colored as the gradient is there.
pub fn added(stops: &[Stop], t: f32) -> (Vec<Stop>, usize) {
    let stop = Stop { at: t.clamp(0.0, 1.0), color: color_at(stops, t) };
    let mut next = stops.to_vec();
    let landed = next.partition_point(|other| other.at <= stop.at);
    next.insert(landed, stop);
    (next, landed)
}

/// `stops` without `ix`, and the stop chosen after: the one before it. Two stops stay.
pub fn removed(stops: &[Stop], ix: usize) -> (Vec<Stop>, usize) {
    if stops.len() <= 2 || ix >= stops.len() {
        return (stops.to_vec(), ix.min(stops.len().saturating_sub(1)));
    }
    let mut next = stops.to_vec();
    next.remove(ix);
    (next, ix.saturating_sub(1))
}

/// The stop whose handle covers `t`: handles reach `reach` either side of their place, and the
/// last one drawn wins where they overlap. `None` when no handle is there.
pub fn stop_at(stops: &[Stop], t: f32, reach: f32) -> Option<usize> {
    stops.iter().rposition(|stop| t >= stop.at - reach && t < stop.at + reach)
}

/// Wires the `Colors` global, which reads the hex typed in a ColorPicker. The host's root
/// `.slint` file exports `Colors`, `ParsedColor` and `ColorSample`.
///
/// Sampling the screen (EyeDropper) is the host's: it sets `Colors.can-sample`, handles
/// `Colors.sample(owner)`, and hands the color back with [`deliver_sample!`](crate::deliver_sample).
///
/// ```ignore
/// ely_slint::wire_colors!(app);
/// ```
#[macro_export]
macro_rules! wire_colors {
    ($app:expr) => {{
        use ::slint::ComponentHandle;
        $app.global::<Colors>().on_parse_hex(|text| match $crate::colors::parse_hex(&text) {
            Ok(rgba) => ParsedColor { ok: true, color: rgba.color() },
            Err(problem) => {
                $crate::log::info!("color picker: {problem}; kept the last color");
                ParsedColor { ok: false, color: ::slint::Color::default() }
            }
        });
    }};
}

/// Hands a color sampled from the screen to the EyeDropper named `owner`.
///
/// ```ignore
/// ely_slint::deliver_sample!(app, "sample", slint::Color::from_rgb_u8(55, 114, 187));
/// ```
#[macro_export]
macro_rules! deliver_sample {
    ($app:expr, $owner:expr, $color:expr) => {{
        use ::slint::ComponentHandle;
        let colors = $app.global::<Colors>();
        let serial = colors.get_sampled().serial + 1;
        let color: ::slint::Color = $color;
        $crate::log::info!("eye dropper: picked {}", $crate::colors::hex($crate::colors::Rgba::of(color)));
        colors.set_sampled(ColorSample { owner: ::slint::SharedString::from($owner), color, serial });
    }};
}

/// Wires the `Gradients` global, which every GradientEditor asks. The host's root `.slint` file
/// exports `Gradients`, `GradientStop` and `GradientEdit`.
///
/// ```ignore
/// ely_slint::wire_gradients!(app);
/// ```
#[macro_export]
macro_rules! wire_gradients {
    ($app:expr) => {{
        use ::slint::{ComponentHandle, Model};
        fn take(stops: &::slint::ModelRc<GradientStop>) -> Vec<$crate::colors::Stop> {
            stops.iter().map(|stop| $crate::colors::Stop { at: stop.at, color: $crate::colors::Rgba::of(stop.color) }).collect()
        }
        fn give((stops, chosen): (Vec<$crate::colors::Stop>, usize)) -> GradientEdit {
            let stops: Vec<GradientStop> = stops.into_iter().map(|stop| GradientStop { at: stop.at, color: stop.color.color() }).collect();
            $crate::log::info!("gradient editor: {} stops, stop {chosen} chosen", stops.len());
            GradientEdit { stops: ::slint::ModelRc::new(::slint::VecModel::from(stops)), chosen: chosen as i32 }
        }
        let gradients = $app.global::<Gradients>();
        gradients.on_added(|stops, t| give($crate::colors::added(&take(&stops), t)));
        gradients.on_moved(|stops, ix, t| give($crate::colors::moved(&take(&stops), ix.max(0) as usize, t)));
        gradients.on_removed(|stops, ix| give($crate::colors::removed(&take(&stops), ix.max(0) as usize)));
        gradients.on_stop_at(|stops, t, reach| $crate::colors::stop_at(&take(&stops), t, reach).map_or(-1, |ix| ix as i32));
        gradients.on_recolored(|stops, ix, color| {
            let mut next = take(&stops);
            if let Some(stop) = next.get_mut(ix.max(0) as usize) {
                stop.color = $crate::colors::Rgba::of(color);
            }
            give((next, ix.max(0) as usize))
        });
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    const BLACK: Rgba = Rgba { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };
    const WHITE: Rgba = Rgba { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };

    fn stops() -> Vec<Stop> {
        vec![Stop { at: 0.0, color: BLACK }, Stop { at: 1.0, color: WHITE }]
    }

    #[test]
    fn hex_reads_three_four_six_and_eight_digits() {
        assert_eq!(hex(parse_hex("#0af").unwrap()), "#00aaff");
        assert_eq!(hex(parse_hex("3772bb").unwrap()), "#3772bb");
        assert_eq!(hex(parse_hex("#3772bb80").unwrap()), "#3772bb80");
        assert_eq!(hex(parse_hex("#0008").unwrap()), "#00000088");
        assert!(parse_hex("#12345").is_err());
        assert!(parse_hex("#zzzzzz").is_err());
        assert!(parse_hex("#+12345").is_err(), "a sign is no digit");
    }

    #[test]
    fn colors_cross_as_bytes() {
        let blue = parse_hex("#3772bb80").unwrap();
        assert_eq!(blue.color(), Color::from_argb_u8(0x80, 0x37, 0x72, 0xbb));
        assert_eq!(hex(Rgba::of(blue.color())), "#3772bb80");
    }

    #[test]
    fn colors_blend_between_stops() {
        let middle = color_at(&stops(), 0.5);
        assert!((middle.r - 0.5).abs() < 0.05);
        assert_eq!(color_at(&stops(), -1.0), BLACK);
    }

    #[test]
    fn adding_and_moving_keep_stops_sorted() {
        let (three, ix) = added(&stops(), 0.25);
        assert_eq!((three.len(), ix, three[1].at), (3, 1, 0.25));
        let (swapped, landed) = moved(&three, 0, 0.5);
        assert_eq!(landed, 1);
        assert_eq!(swapped.iter().map(|stop| stop.at).collect::<Vec<_>>(), [0.25, 0.5, 1.0]);
    }

    #[test]
    fn the_last_handle_drawn_wins() {
        let (three, _) = added(&stops(), 0.05);
        assert_eq!(stop_at(&three, 0.03, 0.04), Some(1));
        assert_eq!(stop_at(&three, -0.03, 0.04), Some(0));
        assert_eq!(stop_at(&three, 0.5, 0.04), None);
        assert_eq!(stop_at(&three, 0.97, 0.04), Some(2));
    }

    #[test]
    fn removing_keeps_two_and_chooses_the_one_before() {
        let (three, _) = added(&stops(), 0.25);
        let (two, chosen) = removed(&three, 1);
        assert_eq!((two.len(), chosen), (2, 0));
        assert_eq!(removed(&two, 1), (two.clone(), 1));
    }
}
