//! What `wire_paths!` hands the `Paths` global: the system's file dialog, after
//! ely-core/src/forms/path.rs (`choose`, `picked`). The host opens the dialog it uses and
//! answers through [`Answer`].

use std::fmt::Display;
use std::path::PathBuf;

use slint::SharedString;

/// The way back to the field that asked. Answer once, from any thread.
pub struct Answer {
    owner: SharedString,
    deliver: Box<dyn FnOnce(Vec<SharedString>) + Send>,
}

impl Answer {
    #[doc(hidden)]
    pub fn new(owner: SharedString, deliver: impl FnOnce(Vec<SharedString>) + Send + 'static) -> Self {
        log::info!("path input: dialog opened");
        Self { owner, deliver: Box::new(deliver) }
    }

    /// The field that asked.
    pub fn owner(&self) -> &str {
        &self.owner
    }

    /// The dialog's answer: `None` when cancelled. The field takes UTF-8 paths only.
    pub fn send(self, paths: Option<Vec<PathBuf>>) {
        let Some(paths) = paths else {
            log::info!("path input: dialog cancelled");
            return;
        };
        if paths.is_empty() {
            log::error!("path input: dialog chose nothing");
            return;
        }
        log::info!("path input: chose {} paths", paths.len());
        let texts = utf8(paths);
        if texts.is_empty() {
            return;
        }
        (self.deliver)(texts);
    }

    /// The dialog failed.
    pub fn fail(self, error: impl Display) {
        log::error!("path input: dialog failed: {error:#}");
    }
}

/// The paths as text; a path that is not UTF-8 is logged and left out, so the field keeps its text.
pub fn utf8(paths: Vec<PathBuf>) -> Vec<SharedString> {
    paths
        .into_iter()
        .filter_map(|path| match path.to_str() {
            Some(text) => Some(SharedString::from(text)),
            None => {
                log::error!("path input: {} is not UTF-8, so the field keeps its text", path.display());
                None
            }
        })
        .collect()
}

/// Wires `Paths`, which every PathInput asks: `dialog(request, answer)` opens the host's file
/// dialog for a `PathRequest` and calls `answer.send(paths)` when it closes, now or later. The
/// host's root `.slint` file exports `Paths` and `PathRequest`.
///
/// ```ignore
/// ely_slint::wire_paths!(app, |request: PathRequest, answer: ely_slint::paths::Answer| {
///     let dialog = rfd::FileDialog::new().set_title(request.prompt.as_str());
///     answer.send(if request.directories { dialog.pick_folder() } else { dialog.pick_file() }.map(|p| vec![p]));
/// });
/// ```
#[macro_export]
macro_rules! wire_paths {
    ($app:expr, $dialog:expr) => {{
        let weak = $app.as_weak();
        let dialog = $dialog;
        $app.global::<Paths>().on_choose(move |request| {
            let weak = weak.clone();
            let owner = request.owner.clone();
            let answer = $crate::paths::Answer::new(request.owner.clone(), move |texts| {
                let sent = weak.upgrade_in_event_loop(move |app| {
                    let paths = app.global::<Paths>();
                    paths.set_answered_owner(owner);
                    paths.set_answered(::slint::ModelRc::new(::slint::VecModel::from(texts)));
                    paths.set_answers(paths.get_answers() + 1);
                });
                if let Err(error) = sent {
                    ::log::error!("path input: window closed first: {error}");
                }
            });
            dialog(request, answer);
        });
    }};
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::utf8;

    #[test]
    fn keeps_utf8_paths() {
        assert_eq!(utf8(vec![PathBuf::from("/tmp/a b")]), vec!["/tmp/a b"]);
    }

    #[cfg(unix)]
    #[test]
    fn drops_paths_that_are_not_utf8() {
        use std::ffi::OsStr;
        use std::os::unix::ffi::OsStrExt;
        let bad = PathBuf::from(OsStr::from_bytes(b"/tmp/\xff"));
        assert!(utf8(vec![bad]).is_empty());
    }
}
