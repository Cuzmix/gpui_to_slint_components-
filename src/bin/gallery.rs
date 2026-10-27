//! The Ely gallery: `cargo run --bin gallery`.

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let gallery = Gallery::new()?;
    ely_slint::wire_text_format!(gallery);
    ely_slint::wire_links!(gallery);
    let platform = ely_slint::Platform::current();
    let spell = |keys: &str| ely_slint::shortcut_text(keys, platform).unwrap_or_default();
    gallery.set_save_shortcut(spell("cmd-s").into());
    gallery.set_find_shortcut(spell("cmd-shift-f").into());
    gallery.on_clicked(|name| eprintln!("gallery: {name} clicked"));
    let weak = gallery.as_weak();
    gallery.global::<FocusSteps>().on_step(move |forward| {
        if let Some(gallery) = weak.upgrade() {
            ely_slint::focus::step(gallery.window(), forward);
        }
    });
    gallery.run()
}
