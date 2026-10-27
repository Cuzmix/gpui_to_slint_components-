//! What the file fields cannot do in `.slint`: look at the files a drop or a dialog brings.
//! Ported from ely-core `forms/files.rs` (`dropped`, `taken`, `file_name`) and `forms/avatar.rs`
//! (`picture`).
//!
//! Slint 1.18 hands no files dragged in from other apps to a window (gap G64), so the host
//! routes them: it names the target under the drag and calls [`hover_files!`](crate::hover_files),
//! [`drop_files!`](crate::drop_files) and [`leave_files!`](crate::leave_files).

use std::path::{Path, PathBuf};

/// Files a picture may be, by extension.
pub const PICTURES: [&str; 5] = ["png", "jpg", "jpeg", "gif", "webp"];

/// The files a drop brings, or why it is refused.
pub fn dropped(paths: &[PathBuf], multiple: bool) -> Result<Vec<PathBuf>, &'static str> {
    if paths.is_empty() {
        return Err("nothing was dropped");
    }
    if !multiple && paths.len() > 1 {
        return Err("one file at a time");
    }
    if paths.iter().any(|path| !path.is_file()) {
        return Err("files only, not folders");
    }
    Ok(paths.to_vec())
}

/// The files a drop or a dialog brings when each is of one of `kinds`, by extension in any case,
/// or why they are refused; no kinds takes any file.
pub fn taken<S: AsRef<str>>(paths: &[PathBuf], multiple: bool, kinds: &[S]) -> Result<Vec<PathBuf>, &'static str> {
    let paths = dropped(paths, multiple)?;
    let fits = |path: &PathBuf| {
        path.extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| kinds.iter().any(|kind| kind.as_ref().eq_ignore_ascii_case(extension)))
    };
    if !kinds.is_empty() && !paths.iter().all(fits) {
        return Err("not a kind this takes");
    }
    Ok(paths)
}

/// The one picture a drop or a pick brings, or why it is refused.
pub fn picture(paths: &[PathBuf]) -> Result<PathBuf, &'static str> {
    let [path] = paths else {
        return Err(if paths.is_empty() { "nothing came" } else { "one picture at a time" });
    };
    if !path.is_file() {
        return Err("files only, not folders");
    }
    let kind = path.extension().and_then(|kind| kind.to_str()).map(str::to_lowercase);
    if !kind.is_some_and(|kind| PICTURES.contains(&kind.as_str())) {
        return Err("pictures only: png, jpg, gif or webp");
    }
    Ok(path.clone())
}

/// The file's name, or the whole path for a root such as `C:\`.
pub fn file_name(path: &Path) -> String {
    match path.file_name() {
        Some(name) => name.to_string_lossy().into_owned(),
        None => path.display().to_string(),
    }
}

/// Wires the `Files` global, which every FileInput and DropZone asks. The host's root `.slint`
/// file exports `Files` and `FileDrag`.
///
/// ```ignore
/// ely_slint::wire_files!(app);
/// ```
#[macro_export]
macro_rules! wire_files {
    ($app:expr) => {{
        use ::slint::{ComponentHandle, Model};
        fn paths(model: &::slint::ModelRc<::slint::SharedString>) -> Vec<::std::path::PathBuf> {
            model.iter().map(|path| ::std::path::PathBuf::from(path.as_str())).collect()
        }
        let files = $app.global::<Files>();
        files.on_refusal(|given, multiple, kinds| {
            let kinds: Vec<::slint::SharedString> = kinds.iter().collect();
            match $crate::files::taken(&paths(&given), multiple, &kinds) {
                Ok(_) => ::slint::SharedString::new(),
                Err(reason) => reason.into(),
            }
        });
        files.on_picture_refusal(|given| match $crate::files::picture(&paths(&given)) {
            Ok(_) => ::slint::SharedString::new(),
            Err(reason) => reason.into(),
        });
        files.on_name(|path| $crate::files::file_name(::std::path::Path::new(path.as_str())).into());
        files.on_refused(|what, reason| $crate::log::info!("{what}: refused {reason}"));
        files.on_took(|what, count| $crate::log::info!("{what}: took {count} files"));
    }};
}

/// Files from another app hover the drop target named `owner`.
///
/// ```ignore
/// ely_slint::hover_files!(app, "photos", vec![std::path::PathBuf::from("/tmp/a.png")]);
/// ```
#[macro_export]
macro_rules! hover_files {
    ($app:expr, $owner:expr, $paths:expr) => {{
        use ::slint::ComponentHandle;
        let files = $app.global::<Files>();
        let serial = files.get_drag().serial;
        let paths: Vec<::slint::SharedString> = $crate::paths::utf8($paths);
        files.set_drag(FileDrag { owner: ::slint::SharedString::from($owner), paths: ::slint::ModelRc::new(::slint::VecModel::from(paths)), hovering: true, dropped: false, serial });
    }};
}

/// Files from another app were dropped on the target named `owner`.
#[macro_export]
macro_rules! drop_files {
    ($app:expr, $owner:expr, $paths:expr) => {{
        use ::slint::ComponentHandle;
        let files = $app.global::<Files>();
        let serial = files.get_drag().serial + 1;
        let paths: Vec<::slint::SharedString> = $crate::paths::utf8($paths);
        files.set_drag(FileDrag { owner: ::slint::SharedString::from($owner), paths: ::slint::ModelRc::new(::slint::VecModel::from(paths)), hovering: false, dropped: true, serial });
    }};
}

/// The drag left every target, or was given up.
#[macro_export]
macro_rules! leave_files {
    ($app:expr) => {{
        use ::slint::ComponentHandle;
        let files = $app.global::<Files>();
        let serial = files.get_drag().serial;
        files.set_drag(FileDrag { serial, ..Default::default() });
    }};
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn a_root_shows_its_whole_path() {
        assert_eq!(file_name(Path::new("/")), "/");
        assert_eq!(file_name(Path::new("/notes/plan.md")), "plan.md");
    }

    #[test]
    fn drops_take_files_and_refuse_folders_extras_and_nothing() {
        let dir = std::env::temp_dir().join(format!("ely-slint-drop-{}", std::process::id()));
        fs::create_dir_all(dir.join("folder")).unwrap();
        let (one, two) = (dir.join("one.txt"), dir.join("two.txt"));
        fs::write(&one, "1").unwrap();
        fs::write(&two, "2").unwrap();
        let both = [one.clone(), two.clone()];
        assert_eq!(dropped(&both, true), Ok(both.to_vec()));
        assert_eq!(dropped(&both, false), Err("one file at a time"));
        assert_eq!(dropped(std::slice::from_ref(&one), false), Ok(vec![one.clone()]));
        assert_eq!(dropped(&[one, dir.join("folder")], true), Err("files only, not folders"));
        assert_eq!(dropped(&[], true), Err("nothing was dropped"));
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn a_zone_of_pictures_takes_pictures_in_any_case_and_refuses_the_rest() {
        let dir = std::env::temp_dir().join(format!("ely-slint-kinds-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let [photo, loud, note] = ["dunes.jpg", "STAIR.PNG", "notes.txt"].map(|name| {
            let path = dir.join(name);
            fs::write(&path, "x").unwrap();
            path
        });
        let both = [photo.clone(), loud.clone()];
        assert_eq!(taken(&both, true, &PICTURES), Ok(both.to_vec()));
        let mixed = [photo, note.clone()];
        assert_eq!(taken(&mixed, true, &PICTURES), Err("not a kind this takes"));
        let lone = std::slice::from_ref(&note);
        assert_eq!(taken::<&str>(lone, false, &[]), Ok(lone.to_vec()), "no kinds takes any file");
        fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn an_avatar_takes_one_picture_file() {
        let dir = std::env::temp_dir().join(format!("ely-slint-avatar-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        let (face, notes) = (dir.join("face.PNG"), dir.join("notes.txt"));
        fs::write(&face, b"png").unwrap();
        fs::write(&notes, b"text").unwrap();
        assert_eq!(picture(std::slice::from_ref(&face)), Ok(face.clone()));
        assert_eq!(picture(&[notes]), Err("pictures only: png, jpg, gif or webp"));
        assert_eq!(picture(&[face.clone(), face]), Err("one picture at a time"));
        assert_eq!(picture(std::slice::from_ref(&dir)), Err("files only, not folders"));
        assert_eq!(picture(&[]), Err("nothing came"));
        fs::remove_dir_all(dir).unwrap();
    }
}
