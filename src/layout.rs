//! Layout math the `.slint` components cannot loop over themselves, after
//! `ely-core/src/layout/grid.rs`.

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

#[cfg(test)]
mod tests {
    use super::*;

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
