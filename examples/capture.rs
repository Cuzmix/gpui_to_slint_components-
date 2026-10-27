//! Renders a gallery page to a PNG with Slint's software renderer, no display needed:
//! `cargo run --example capture -- <page slug> <out.png> [dark]`.

use std::rc::Rc;

use slint::platform::software_renderer::{MinimalSoftwareWindow, RepaintBufferType};
use slint::Rgb8Pixel;
use slint::platform::{Platform, WindowAdapter};
use slint::{ComponentHandle, Model, PhysicalSize};

slint::include_modules!();

#[path = "../src/bin/gallery/wire.rs"]
mod wire;

struct Headless(Rc<MinimalSoftwareWindow>);

impl Platform for Headless {
    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, slint::PlatformError> {
        Ok(self.0.clone())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let slug = args.next().unwrap_or_else(|| "typography".into());
    let out = args.next().unwrap_or_else(|| "gallery.png".into());
    let dark = args.next().as_deref() == Some("dark");

    let window = MinimalSoftwareWindow::new(RepaintBufferType::NewBuffer);
    slint::platform::set_platform(Box::new(Headless(window.clone())))?;

    let gallery = Gallery::new()?;
    let _wired = wire::wire(&gallery);
    gallery.set_start_dark(dark);
    let index = gallery.get_pages().iter().position(|page| page.slug == slug).ok_or("no such page")?;
    gallery.set_current(index as i32);

    let (width, height) = (1100u32, gallery.get_capture_height().max(760.0) as u32);
    window.set_size(PhysicalSize::new(width, height));
    gallery.show()?;
    // Let timers, animations and the theme fade settle.
    for _ in 0..120 {
        slint::platform::update_timers_and_animations();
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
    let height = gallery.get_capture_height().max(760.0) as u32;
    window.set_size(PhysicalSize::new(width, height));
    slint::platform::update_timers_and_animations();

    let mut pixels = vec![Rgb8Pixel::default(); (width * height) as usize];
    window.request_redraw();
    window.draw_if_needed(|renderer| {
        renderer.render(&mut pixels, width as usize);
    });

    let file = std::fs::File::create(&out)?;
    let mut encoder = png::Encoder::new(std::io::BufWriter::new(file), width, height);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    let bytes: Vec<u8> = pixels.iter().flat_map(|p| [p.r, p.g, p.b]).collect();
    encoder.write_header()?.write_image_data(&bytes)?;
    println!("wrote {out} ({width}×{height})");
    Ok(())
}
