//! The Ely gallery: `cargo run --bin gallery`.

slint::include_modules!();

mod wire;

fn main() -> Result<(), slint::PlatformError> {
    let gallery = Gallery::new()?;
    let _wired = wire::wire(&gallery);
    let platform = ely_slint::Platform::current();
    let spell = |keys: &str| ely_slint::shortcut_text(keys, platform).unwrap_or_default();
    gallery.set_save_shortcut(spell("cmd-s").into());
    gallery.set_find_shortcut(spell("cmd-shift-f").into());
    let quick: Vec<slint::SharedString> = ["cmd-n", "cmd-o", "cmd-shift-p", "cmd-,"].iter().map(|keys| spell(keys).into()).collect();
    gallery.set_quick_shortcuts(slint::ModelRc::new(slint::VecModel::from(quick)));
    gallery.on_clicked(|name| eprintln!("gallery: {name} clicked"));
    let weak = gallery.as_weak();
    gallery.global::<FocusSteps>().on_step(move |forward| {
        if let Some(gallery) = weak.upgrade() {
            ely_slint::focus::step(gallery.window(), forward);
        }
    });
    gallery.run()
}
