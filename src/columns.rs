//! ColumnShell's widths (ely-core/src/shell/columns.rs): columns side by side as gpui's flex row
//! lays them out, set widths as flex bases that shrink when the shell runs short, one column
//! filling the rest, none narrower than its header's fixed parts.

/// One column as the owner sets it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Column {
    /// A set width, or `None` to fill.
    pub width: Option<f32>,
    /// Its border drags its width, within `min` and `max`.
    pub resizable: bool,
    pub min: f32,
    pub max: f32,
}

/// What the shell's chrome takes in the headers.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Room {
    /// The first header's left padding: room for the system's lights, or 12 px.
    pub first_pad: f32,
    /// Drawn controls leading the first header (macOS's manner), 0 if none.
    pub lead: f32,
    /// Drawn controls ending the last header, 0 if none.
    pub trail: f32,
    /// The last header ends flush with the controls (Windows' manner).
    pub flush: bool,
}

/// The border a column's width is dragged by, or draws its line on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

/// Where a column lies and what its header leaves for the owner's items.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Placed {
    pub x: f32,
    pub width: f32,
    /// The border toward the filling column, if any.
    pub side: Option<Side>,
    /// Its border drags its width: resizable with a side.
    pub resizes: bool,
    /// The header items' room, from the column's left.
    pub items_x: f32,
    pub items_width: f32,
}

const ITEMS_MIN: f32 = 32.0;
const PAD: f32 = 12.0;
const GAP: f32 = 8.0;

/// Each border lies toward the column that fills, so a drag on it moves one width.
pub fn sides(columns: &[Column]) -> Vec<Option<Side>> {
    let fill = columns.iter().position(|column| column.width.is_none());
    let last = columns.len().saturating_sub(1);
    (0..columns.len())
        .map(|ix| match fill {
            Some(fill) if ix > fill => Some(Side::Left),
            Some(fill) if ix == fill => None,
            _ => (ix != last).then_some(Side::Right),
        })
        .collect()
}

/// A flex row's main sizes: `bases` (`None` grows from 0 with factor 1; a set basis neither
/// grows), every item shrinking by its basis when the row runs short, none below its `min`
/// (CSS flexbox's resolution of flexible lengths, as taffy runs it).
pub fn flex_row(bases: &[Option<f32>], mins: &[f32], width: f32) -> Vec<f32> {
    let n = bases.len();
    let base: Vec<f32> = bases.iter().map(|b| b.unwrap_or(0.0)).collect();
    let hypothetical: Vec<f32> = (0..n).map(|ix| base[ix].max(mins[ix])).collect();
    let growing = hypothetical.iter().sum::<f32>() < width;
    let mut target = hypothetical.clone();
    let mut frozen: Vec<bool> = (0..n)
        .map(|ix| {
            let factor = if growing { if bases[ix].is_none() { 1.0 } else { 0.0 } } else { 1.0 };
            factor == 0.0 || (growing && base[ix] > hypothetical[ix]) || (!growing && base[ix] < hypothetical[ix])
        })
        .collect();
    loop {
        let open: Vec<usize> = (0..n).filter(|ix| !frozen[*ix]).collect();
        if open.is_empty() {
            break;
        }
        let used: f32 = (0..n).map(|ix| if frozen[ix] { target[ix] } else { base[ix] }).sum();
        let free = width - used;
        let weights: Vec<f32> = open.iter().map(|ix| if growing { 1.0 } else { base[*ix] }).collect();
        let total: f32 = weights.iter().sum();
        let mut violation = 0.0;
        let mut clamped = vec![false; n];
        for (k, ix) in open.iter().enumerate() {
            let share = if total > 0.0 { free * weights[k] / total } else { 0.0 };
            let unclamped = base[*ix] + share;
            let fixed = unclamped.max(mins[*ix]);
            violation += fixed - unclamped;
            clamped[*ix] = fixed > unclamped;
            target[*ix] = fixed;
        }
        if violation == 0.0 {
            break;
        }
        for ix in open {
            // Only min violations are possible (there is no max).
            if violation > 0.0 && clamped[ix] {
                frozen[ix] = true;
            }
        }
    }
    target
}

/// Lays `columns` out across `width`.
pub fn place(columns: &[Column], width: f32, room: Room) -> Vec<Placed> {
    let count = columns.len();
    let sides = sides(columns);
    let mut mins = Vec::with_capacity(count);
    let mut pads = Vec::with_capacity(count);
    for (ix, column) in columns.iter().enumerate() {
        let (first, last) = (ix == 0, ix + 1 == count);
        // gpui panics on both; here the column keeps its width.
        let resizes = column.resizable
            && if sides[ix].is_none() {
                log::error!("column shell: column {ix} resizes, but no border lies toward a filling column");
                false
            } else if column.min > column.max {
                log::error!("column shell: column {ix}'s limits {} to {} run backwards", column.min, column.max);
                false
            } else {
                true
            };
        let lead = first && room.lead > 0.0;
        let trail = last && room.trail > 0.0;
        let left = if first { room.first_pad } else { PAD } + if lead { room.lead + GAP } else { 0.0 };
        let right = if trail && room.flush { 0.0 } else { PAD } + if trail { room.trail + GAP } else { 0.0 };
        // A resizing column's handle draws its border; another draws a 1 px line.
        let border = if sides[ix].is_some() && !resizes { 1.0 } else { 0.0 };
        mins.push(left + ITEMS_MIN + right + border);
        pads.push((left, right, border, resizes));
    }
    let bases: Vec<Option<f32>> = columns.iter().map(|column| column.width).collect();
    let widths = flex_row(&bases, &mins, width);
    let mut x = 0.0f32;
    let mut placed = Vec::with_capacity(count);
    for ix in 0..count {
        let (left, right, border, resizes) = pads[ix];
        // Edges on whole pixels, as taffy rounds them.
        let (start, end) = (x.round(), (x + widths[ix]).round());
        x += widths[ix];
        let width = end - start;
        let line_left = if sides[ix] == Some(Side::Left) { border } else { 0.0 };
        placed.push(Placed {
            x: start,
            width,
            side: sides[ix],
            resizes,
            items_x: line_left + left,
            items_width: (width - border - left - right).max(0.0),
        });
    }
    placed
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set(width: f32) -> Column {
        Column { width: Some(width), resizable: false, min: 0.0, max: 0.0 }
    }
    const FILL: Column = Column { width: None, resizable: false, min: 0.0, max: 0.0 };
    const ROOM: Room = Room { first_pad: 12.0, lead: 0.0, trail: 0.0, flush: false };

    // Set widths hold, the last column takes the rest.
    #[test]
    fn set_widths_hold_and_the_fill_takes_the_rest() {
        let placed = place(&[set(240.0), set(220.0), FILL], 1000.0, ROOM);
        let spans: Vec<_> = placed.iter().map(|p| (p.x, p.width)).collect();
        assert_eq!(spans, vec![(0.0, 240.0), (240.0, 220.0), (460.0, 540.0)]);
        assert_eq!(placed.iter().map(|p| p.side).collect::<Vec<_>>(), vec![Some(Side::Right), Some(Side::Right), None]);
    }

    // Short, set widths shrink by their bases; the filling column keeps its least width.
    #[test]
    fn a_short_shell_shrinks_set_widths_by_their_bases() {
        let placed = place(&[set(240.0), FILL], 280.0, ROOM);
        // The fill's least width is its pads and items: 12 + 32 + 12.
        assert_eq!(placed[1].width, 56.0);
        assert_eq!(placed[0].width, 224.0);
    }

    // Borders after the fill lie on their left; nothing fills, the last has none.
    #[test]
    fn borders_lie_toward_the_fill() {
        assert_eq!(sides(&[set(1.0), FILL, set(1.0)]), vec![Some(Side::Right), None, Some(Side::Left)]);
        assert_eq!(sides(&[set(1.0), set(1.0)]), vec![Some(Side::Right), None]);
    }
}
