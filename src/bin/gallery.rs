//! The Ely gallery: `cargo run --bin gallery`.

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let gallery = Gallery::new()?;
    let platform = ely_slint::Platform::current();
    let spell = |keys: &str| ely_slint::shortcut_text(keys, platform).unwrap_or_default();
    gallery.set_save_shortcut(spell("cmd-s").into());
    gallery.set_find_shortcut(spell("cmd-shift-f").into());
    gallery.on_clicked(|name| eprintln!("gallery: {name} clicked"));
    gallery.run()
}
