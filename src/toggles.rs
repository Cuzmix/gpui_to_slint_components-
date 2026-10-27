//! What `wire_toggles!` hands the `Toggles` global: a ToggleGroup's selection, after
//! ely-core/src/buttons/group.rs (`pressed`).

use slint::SharedString;

/// The selection after pressing `value`: an item that is on turns off; one that is off joins
/// the selection with `multiple`, else replaces it.
pub fn pressed(selected: &[SharedString], value: &SharedString, multiple: bool) -> Vec<SharedString> {
    let on = selected.contains(value);
    match (multiple, on) {
        (_, true) => selected.iter().filter(|v| *v != value).cloned().collect(),
        (true, false) => selected.iter().chain([value]).cloned().collect(),
        (false, false) => vec![value.clone()],
    }
}

/// Wires `Toggles`, which every ToggleGroup asks. The host's root `.slint` file exports
/// `Toggles`.
///
/// ```ignore
/// ely_slint::wire_toggles!(app);
/// ```
#[macro_export]
macro_rules! wire_toggles {
    ($app:expr) => {{
        use ::slint::Model;
        let toggles = $app.global::<Toggles>();
        toggles.on_contains(|selected, value| selected.iter().any(|v| v == value));
        toggles.on_pressed(|selected, value, multiple| {
            let selected: Vec<::slint::SharedString> = selected.iter().collect();
            let next = $crate::toggles::pressed(&selected, &value, multiple);
            ::slint::ModelRc::new(::slint::VecModel::from(next))
        });
    }};
}

#[cfg(test)]
mod tests {
    use slint::SharedString;

    use super::pressed;

    fn values(list: &[&'static str]) -> Vec<SharedString> {
        list.iter().map(|v| SharedString::from(*v)).collect()
    }

    #[test]
    fn single_replaces_multiple_adds_and_pressing_again_clears() {
        let b = SharedString::from("b");
        assert_eq!(pressed(&values(&["a"]), &b, false), values(&["b"]));
        assert_eq!(pressed(&values(&["a"]), &b, true), values(&["a", "b"]));
        assert_eq!(pressed(&values(&["a", "b"]), &b, true), values(&["a"]));
        assert_eq!(pressed(&values(&["b"]), &b, false), values(&[]));
    }
}
