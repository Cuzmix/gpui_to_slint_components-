//! Layout math the `.slint` components cannot loop over themselves, after
//! `ely-core/src/layout/grid.rs` and `split.rs`.

/// Columns for `width`, each at least `min`, `gap` apart; at least one.
pub fn columns_for(width: f32, min: f32, gap: f32) -> u16 {
    let fit = ((width + gap) / (min + gap)).floor();
    fit.clamp(1.0, f32::from(u16::MAX)) as u16
}

/// How many items `item` wide with `gap` between fit across `width`, at least one, and the
/// inset that centers them.
pub fn fit(width: f32, item: f32, gap: f32) -> (usize, f32) {
    let count = columns_for(width, item, gap) as usize;
    let used = (item + gap) * count as f32 - gap;
    (count, ((width - used) / 2.0).max(0.0))
}

/// Index of the shortest column; ties go left. A NaN height never wins.
pub fn shortest(heights: &[f32]) -> usize {
    heights
        .iter()
        .enumerate()
        .min_by(|a, b| a.1.total_cmp(b.1))
        .map(|(ix, _)| ix)
        .expect("at least one column")
}

/// Each item's column and top: it joins the column filled least so far, `gap` below the item
/// before it there. Fewer than one column counts as one.
pub fn masonry(heights: &[f32], columns: usize, gap: f32) -> Vec<(usize, f32)> {
    let mut filled = vec![0.0; columns.max(1)];
    heights
        .iter()
        .map(|height| {
            let column = shortest(&filled);
            let top = filled[column];
            filled[column] += height + gap;
            (column, top)
        })
        .collect()
}

/// The height of the tallest masonry column.
pub fn masonry_height(heights: &[f32], columns: usize, gap: f32) -> f32 {
    let mut filled = vec![0.0f32; columns.max(1)];
    let mut used = vec![false; columns.max(1)];
    for height in heights {
        let column = shortest(&filled);
        filled[column] += height + gap;
        used[column] = true;
    }
    filled
        .iter()
        .zip(used)
        .map(|(filled, used)| if used { filled - gap } else { 0.0 })
        .fold(0.0, f32::max)
}

/// Weights are finite and non-negative, with a finite, positive sum; returns the sum.
pub fn check_weights(weights: &[f32]) -> Result<f32, String> {
    if !weights.iter().all(|weight| weight.is_finite() && *weight >= 0.0) {
        return Err(format!("split weights {weights:?} must be finite and non-negative"));
    }
    let total: f32 = weights.iter().sum();
    if !(total.is_finite() && total > 0.0) {
        return Err(format!("split weights {weights:?} need a finite, positive sum"));
    }
    Ok(total)
}

/// A split's shares for `count` panes: `weights` normalized, or even shares (logged) when
/// there are none, too few or too many, or they do not check.
pub fn split_shares(weights: &[f32], count: usize) -> Vec<f32> {
    let even = vec![1.0 / count.max(1) as f32; count];
    if weights.is_empty() {
        return even;
    }
    if weights.len() != count {
        log::error!("split pane: {} sizes for {count} panes; even shares", weights.len());
        return even;
    }
    match check_weights(weights) {
        Ok(total) => weights.iter().map(|weight| weight / total).collect(),
        Err(error) => {
            log::error!("split pane: {error}; even shares");
            even
        }
    }
}

/// Moves divider `ix` by `delta` of the whole, keeping each pane at `floor` or more.
/// Returns nothing for a divider the shares do not have.
pub fn split_shift(start: &[f32], ix: usize, delta: f32, floor: f32) -> Vec<f32> {
    if ix + 1 >= start.len() {
        log::error!("split pane: divider {ix} of {} panes", start.len());
        return Vec::new();
    }
    let pair = start[ix] + start[ix + 1];
    let floor = floor.min(pair / 2.0);
    let left = (start[ix] + delta).clamp(floor, pair - floor);
    let mut next = start.to_vec();
    next[ix] = left;
    next[ix + 1] = pair - left;
    next
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn weights_must_be_finite_non_negative_and_sum_positive() {
        assert_eq!(check_weights(&[1.0, 3.0]).unwrap(), 4.0);
        assert_eq!(check_weights(&[0.0, 1.0]).unwrap(), 1.0);
        assert!(check_weights(&[0.0, 0.0]).is_err());
        assert!(check_weights(&[-1.0, 2.0]).is_err());
        assert!(check_weights(&[f32::NAN, 1.0]).is_err());
        assert!(check_weights(&[f32::MAX, f32::MAX]).is_err());
    }

    #[test]
    fn shares_normalize_or_fall_back_to_even() {
        assert_eq!(split_shares(&[1.0, 3.0], 2), vec![0.25, 0.75]);
        assert_eq!(split_shares(&[], 4), vec![0.25; 4]);
        assert_eq!(split_shares(&[1.0], 2), vec![0.5, 0.5]);
        assert_eq!(split_shares(&[-1.0, 2.0], 2), vec![0.5, 0.5]);
    }

    #[test]
    fn shift_moves_one_boundary_and_respects_the_floor() {
        let moved = split_shift(&[0.5, 0.5], 0, 0.1, 0.1);
        assert!((moved[0] - 0.6).abs() < 1e-6 && (moved[1] - 0.4).abs() < 1e-6);
        let floored = split_shift(&[0.5, 0.5], 0, -0.9, 0.2);
        assert!((floored[0] - 0.2).abs() < 1e-6);
        let three = split_shift(&[0.2, 0.3, 0.5], 1, 0.1, 0.05);
        assert!((three[1] - 0.4).abs() < 1e-6 && (three[2] - 0.4).abs() < 1e-6);
        assert!((three[0] - 0.2).abs() < 1e-6);
        assert!(split_shift(&[0.5, 0.5], 1, 0.1, 0.1).is_empty());
    }

    #[test]
    fn a_nan_height_never_stops_the_shortest_column() {
        assert_eq!(shortest(&[f32::NAN, 4.0, 2.0]), 2);
    }

    #[test]
    fn what_fits_sits_centered() {
        assert_eq!(fit(100.0, 3.0, 2.0), (20, 1.0));
        assert_eq!(fit(98.0, 3.0, 2.0), (20, 0.0));
        assert_eq!(fit(2.0, 3.0, 2.0), (1, 0.0));
    }

    #[test]
    fn columns_fit_the_width() {
        assert_eq!(columns_for(1000.0, 220.0, 16.0), 4);
        assert_eq!(columns_for(100.0, 220.0, 16.0), 1);
        assert_eq!(columns_for(0.0, 220.0, 16.0), 1);
    }

    #[test]
    fn shortest_prefers_the_left_on_ties() {
        assert_eq!(shortest(&[3.0, 1.0, 1.0]), 1);
        assert_eq!(shortest(&[0.0, 0.0]), 0);
    }

    #[test]
    fn masonry_fills_the_shortest_column() {
        let spots = masonry(&[72.0, 120.0, 56.0, 96.0, 140.0], 3, 10.0);
        assert_eq!(spots, vec![(0, 0.0), (1, 0.0), (2, 0.0), (2, 66.0), (0, 82.0)]);
        assert_eq!(masonry_height(&[72.0, 120.0, 56.0, 96.0, 140.0], 3, 10.0), 222.0);
        assert_eq!(masonry_height(&[50.0], 3, 10.0), 50.0);
        assert_eq!(masonry_height(&[], 3, 10.0), 0.0);
    }
}
