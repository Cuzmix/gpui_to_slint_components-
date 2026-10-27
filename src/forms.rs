//! What the form structure cannot do in `.slint`: log a submit. Ported from ely-core
//! `forms/structure.rs` (`Form`'s log line).

/// Wires the `Forms` global, which every Form asks. The host's root `.slint` file exports
/// `Forms`.
///
/// ```ignore
/// ely_slint::wire_forms!(app);
/// ```
#[macro_export]
macro_rules! wire_forms {
    ($app:expr) => {{
        use ::slint::ComponentHandle;
        $app.global::<Forms>().on_submitted(|owner| $crate::log::info!("form {owner:?}: submitted"));
    }};
}
