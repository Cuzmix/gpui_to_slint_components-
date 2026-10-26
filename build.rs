// Compiles the gallery, which imports every component, so `cargo build` checks the whole
// .slint library and the `gallery` binary can run it.
fn main() {
    let config = slint_build::CompilerConfiguration::new();
    slint_build::compile_with_config("ui/gallery.slint", config).expect("ui/gallery.slint compiles");
}
