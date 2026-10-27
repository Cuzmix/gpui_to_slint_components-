//! What option lists cannot do in `.slint`: walk their rows. Ported from ely-core
//! `forms/options.rs` (`step`). The `Choices` global in `ui/components/options.slint` reaches it
//! through a pure callback the host wires with [`wire_options!`](crate::wire_options).

/// The next row from `from` that is not disabled, stepping by `by` and wrapping; `from` when
/// every row is disabled (ely `step`).
pub fn step(disabled: &[bool], from: usize, by: isize) -> usize {
    let count = disabled.len() as isize;
    let mut at = from as isize;
    for _ in 0..count {
        at = (at + by).rem_euclid(count);
        if !disabled[at as usize] {
            return at as usize;
        }
    }
    from
}

/// Wires the `Choices` global of a component that shows option lists.
#[macro_export]
macro_rules! wire_options {
    ($app:expr) => {{
        use ::slint::Model;
        $app.global::<Choices>().on_step(|rows, from, by| {
            let disabled: Vec<bool> = rows.iter().map(|row| row.disabled).collect();
            if disabled.is_empty() {
                return from;
            }
            $crate::options::step(&disabled, from.max(0) as usize, by as isize) as i32
        });
        $app.global::<Choices>().on_plain(|labels| {
            let rows: Vec<Choice> = labels.iter().map(|label| Choice { value: label.clone(), label, ..Default::default() }).collect();
            ::slint::ModelRc::new(::slint::VecModel::from(rows))
        });
    }};
}

#[cfg(test)]
mod tests {
    use super::step;

    #[test]
    fn stepping_wraps_and_skips_disabled_rows() {
        let rows = [false, true, false];
        assert_eq!(step(&rows, 0, 1), 2);
        assert_eq!(step(&rows, 2, 1), 0);
        assert_eq!(step(&rows, 0, -1), 2);
        assert_eq!(step(&[true], 0, 1), 0);
    }
}
