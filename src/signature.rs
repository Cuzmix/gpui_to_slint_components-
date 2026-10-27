//! What SignaturePad cannot do in `.slint`: grow a stroke point by point and bend it into a
//! path. Ported from ely-core `forms/signature.rs` (`Drawing::reach`, `local`, `ink`). Points are
//! logical pixels from the pad's top left corner.

/// `at` from the pad's corner, kept inside a pad `width` by `height`.
pub fn local(at: (f32, f32), width: f32, height: f32) -> (f32, f32) {
    (at.0.max(0.0).min(width), at.1.max(0.0).min(height))
}

/// `stroke` with `at` added, unless the pen has not moved.
pub fn reach(mut stroke: Vec<(f32, f32)>, at: (f32, f32)) -> Vec<(f32, f32)> {
    if stroke.last() != Some(&at) {
        stroke.push(at);
    }
    stroke
}

/// Path commands for a line through `points`, bent through their midpoints so it reads smooth;
/// empty under two points.
pub fn ink(points: &[(f32, f32)]) -> String {
    let [first, .., last] = points else {
        return String::new();
    };
    let mut path = format!("M {} {}", first.0, first.1);
    for pair in points.windows(2) {
        let middle = ((pair[0].0 + pair[1].0) / 2.0, (pair[0].1 + pair[1].1) / 2.0);
        path.push_str(&format!(" Q {} {} {} {}", pair[0].0, pair[0].1, middle.0, middle.1));
    }
    path.push_str(&format!(" L {} {}", last.0, last.1));
    path
}

/// Wires the `Signatures` global, which every SignaturePad asks. The host's root `.slint` file
/// exports `Signatures` and `PenPoint`.
///
/// ```ignore
/// ely_slint::wire_signatures!(app);
/// ```
#[macro_export]
macro_rules! wire_signatures {
    ($app:expr) => {{
        use ::slint::{ComponentHandle, Model};
        fn points(stroke: &::slint::ModelRc<PenPoint>) -> Vec<(f32, f32)> {
            stroke.iter().map(|point| (point.x, point.y)).collect()
        }
        let signatures = $app.global::<Signatures>();
        signatures.on_reach(|stroke, x, y, width, height| {
            let at = $crate::signature::local((x, y), width, height);
            let next: Vec<PenPoint> = $crate::signature::reach(points(&stroke), at).into_iter().map(|(x, y)| PenPoint { x, y }).collect();
            ::slint::ModelRc::new(::slint::VecModel::from(next))
        });
        signatures.on_with_stroke(|strokes, stroke| {
            let mut next: Vec<::slint::ModelRc<PenPoint>> = strokes.iter().collect();
            next.push(stroke);
            $crate::log::info!("signature pad: {} strokes", next.len());
            ::slint::ModelRc::new(::slint::VecModel::from(next))
        });
        signatures.on_ink(|stroke| $crate::signature::ink(&points(&stroke)).into());
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn points_stay_inside_the_pad() {
        assert_eq!(local((20.0, 20.0), 100.0, 50.0), (20.0, 20.0));
        assert_eq!(local((-15.0, 70.0), 100.0, 50.0), (0.0, 50.0));
    }

    #[test]
    fn a_stroke_needs_two_points_for_a_line() {
        assert_eq!(ink(&[(1.0, 1.0)]), "");
        assert_eq!(ink(&[(1.0, 1.0), (9.0, 5.0)]), "M 1 1 Q 1 1 5 3 L 9 5");
        assert_eq!(reach(vec![(1.0, 1.0)], (1.0, 1.0)), [(1.0, 1.0)], "a still pen adds nothing");
    }
}
