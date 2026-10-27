//! Opening an address in the system's handler, as gpui's `App::open_url`.

use std::io;
use std::process::{Command, Stdio};

/// Hands `url` to the system: `open` on macOS, `start` on Windows, `xdg-open` elsewhere.
/// Returns once the opener has started; it does not wait for the browser.
pub fn open_url(url: &str) -> io::Result<()> {
    let mut command = if cfg!(target_os = "macos") {
        let mut command = Command::new("open");
        command.arg(url);
        command
    } else if cfg!(target_os = "windows") {
        let mut command = Command::new("cmd");
        command.args(["/C", "start", "", url]);
        command
    } else {
        let mut command = Command::new("xdg-open");
        command.arg(url);
        command
    };
    command
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map(drop)
}

/// Wires the `Links` global of `app` to [`open_url`]. The host's root `.slint` file exports
/// `Links`, and the generated types are in scope where this is called.
///
/// ```ignore
/// let app = App::new()?;
/// ely_slint::wire_links!(app);
/// ```
#[macro_export]
macro_rules! wire_links {
    ($app:expr) => {{
        $app.global::<Links>().on_open(|url| {
            if let Err(error) = $crate::open::open_url(&url) {
                $crate::log::error!("link: could not open {url}: {error}");
            }
        });
    }};
}
