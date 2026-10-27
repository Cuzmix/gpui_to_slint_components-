// FileInput, DropZone and UploadList, after ely-core/src/forms/files.rs and upload.rs.
#[macro_use]
mod common;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use common::*;
use i_slint_backend_testing::ElementHandle;
use slint::platform::Key;
use slint::{ComponentHandle, Model, SharedString};

slint::slint! {
    import { FileInput, DropZone, UploadList, UploadState, Upload, Files, FileDrag, Paths, PathRequest, TextFormat, PlatformStyle, RollCell } from "ui/index.slint";
    export { Files, FileDrag, Paths, PathRequest, TextFormat, PlatformStyle, RollCell }
    export component Harness inherits Window {
        width: 900px;
        height: 500px;
        in-out property <[string]> one-paths;
        in-out property <[string]> many-paths: ["/repo/Cargo.toml", "/repo/README.md", "/repo/AGENTS.md"];
        in-out property <int> changes;
        in-out property <[string]> taken;
        in-out property <int> takes;
        in-out property <string> acted;
        out property <bool> one-over: one.drag-over;
        out property <bool> one-takes: one.takes-drag;
        out property <bool> zone-over: zone.drag-over;
        out property <bool> zone-takes: zone.takes-drag;
        public function focus-one() { one.focus(); }
        one := FileInput {
            x: 10px; y: 10px; width: 260px;
            owner: "one";
            paths <=> root.one-paths;
            paths-changed => { root.changes += 1; }
        }
        FileInput {
            x: 10px; y: 50px; width: 260px;
            owner: "many";
            multiple: true;
            paths <=> root.many-paths;
            paths-changed => { root.changes += 1; }
        }
        zone := DropZone {
            x: 10px; y: 100px; width: 420px;
            owner: "zone";
            multiple: true;
            kinds: Files.pictures;
            hint: "Pictures only";
            dropped(paths) => { root.taken = paths; root.takes += 1; }
        }
        UploadList {
            x: 450px; y: 10px; width: 400px;
            can-cancel: true;
            can-retry: true;
            can-remove: true;
            uploads: [
                { name: "report.pdf", bytes: 2400000, state: UploadState.uploading, share: 0.4 },
                { name: "photo.png", bytes: 2400000, state: UploadState.done },
                { name: "notes.txt", bytes: 12, state: UploadState.failed, reason: "Network lost" },
            ];
            cancel(ix) => { root.acted = "cancel " + ix; }
            retry(ix) => { root.acted = "retry " + ix; }
            remove(ix) => { root.acted = "remove " + ix; }
        }
    }
}

type Asked = Rc<RefCell<Vec<(String, bool)>>>;
type Answer = Rc<RefCell<Vec<PathBuf>>>;

fn harness() -> (Harness, Asked, Answer) {
    let h = Harness::new().unwrap();
    ely_slint::wire_files!(h);
    ely_slint::wire_text_format!(h);
    let (asked, answer): (Asked, Answer) = Default::default();
    let (log, give) = (asked.clone(), answer.clone());
    ely_slint::wire_paths!(h, move |request: PathRequest, reply: ely_slint::paths::Answer| {
        log.borrow_mut().push((request.owner.to_string(), request.multiple));
        reply.send(Some(give.borrow().clone()));
    });
    show(&h);
    (h, asked, answer)
}

// Runs what the dialog's answer queued for the event loop.
fn settle() {
    slint::invoke_from_event_loop(|| slint::quit_event_loop().unwrap()).unwrap();
    slint::run_event_loop().unwrap();
    advance(0);
}

// Real files in a fresh folder, by name.
fn files(names: &[&str]) -> (PathBuf, Vec<PathBuf>) {
    static COUNT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let n = COUNT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("ely-slint-files-{}-{n}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let paths = names
        .iter()
        .map(|name| {
            let path = dir.join(name);
            std::fs::write(&path, "x").unwrap();
            path
        })
        .collect();
    (dir, paths)
}

fn texts(model: slint::ModelRc<SharedString>) -> Vec<String> {
    model.iter().map(|text| text.to_string()).collect()
}

fn value_of(h: &Harness, label: &str) -> String {
    ElementHandle::find_by_accessible_label(h, label).next().unwrap().accessible_value().unwrap().to_string()
}

fn a_press_opens_the_dialog_and_fills_the_field() {
    let (h, asked, answer) = harness();
    assert_eq!(value_of(&h, "Choose a file…"), "");
    *answer.borrow_mut() = vec![PathBuf::from("/notes/plan.md")];
    click(h.window(), 140.0, 24.0);
    settle();
    assert_eq!(asked.borrow().as_slice(), &[("one".to_string(), false)]);
    assert_eq!(texts(h.get_one_paths()), ["/notes/plan.md"]);
    assert_eq!(value_of(&h, "Choose a file…"), "plan.md");
    assert_eq!(h.get_changes(), 1);
    // Enter, pressed and released, opens it too.
    h.invoke_focus_one();
    key_down(h.window(), Key::Return);
    assert_eq!(asked.borrow().len(), 1, "a press only arms it");
    key_up(h.window(), Key::Return);
    assert_eq!(asked.borrow().len(), 2);
}

fn several_show_the_first_and_a_count_and_clear_empties() {
    let (h, _, _) = harness();
    assert_eq!(value_of(&h, "Choose files…"), "Cargo.toml +2");
    click_label(&h, "Clear");
    assert!(h.get_many_paths().row_count() == 0);
    assert_eq!(h.get_changes(), 1);
    assert_eq!(value_of(&h, "Choose files…"), "");
}

fn drops_tint_and_take_what_fits() {
    let (h, _, _) = harness();
    let (dir, paths) = files(&["a.txt", "b.txt"]);
    ely_slint::hover_files!(h, "one", paths.clone());
    assert!(h.get_one_over() && !h.get_one_takes(), "two files on a one-file field");
    ely_slint::hover_files!(h, "one", vec![paths[0].clone()]);
    assert!(h.get_one_takes());
    ely_slint::drop_files!(h, "one", vec![paths[0].clone()]);
    advance(0);
    assert!(!h.get_one_over());
    assert_eq!(texts(h.get_one_paths()), [paths[0].to_str().unwrap()]);
    ely_slint::drop_files!(h, "one", vec![dir.clone()]);
    advance(0);
    assert_eq!(h.get_changes(), 1, "a folder is refused");
    ely_slint::hover_files!(h, "one", paths.clone());
    ely_slint::leave_files!(h);
    assert!(!h.get_one_over());
    std::fs::remove_dir_all(dir).unwrap();
}

fn a_zone_of_pictures_refuses_the_rest() {
    let (h, asked, answer) = harness();
    let (dir, paths) = files(&["dunes.jpg", "STAIR.PNG", "notes.txt"]);
    ely_slint::hover_files!(h, "zone", paths.clone());
    assert!(h.get_zone_over() && !h.get_zone_takes());
    ely_slint::drop_files!(h, "zone", paths.clone());
    advance(0);
    assert_eq!(h.get_takes(), 0);
    ely_slint::drop_files!(h, "zone", paths[..2].to_vec());
    advance(0);
    assert_eq!((h.get_takes(), texts(h.get_taken()).len()), (1, 2));
    // Browse goes through the same gate.
    *answer.borrow_mut() = vec![paths[2].clone()];
    click_label(&h, "Browse…");
    settle();
    assert_eq!(asked.borrow().last().unwrap(), &("zone".to_string(), true));
    assert_eq!(h.get_takes(), 1);
    *answer.borrow_mut() = vec![paths[0].clone()];
    click_label(&h, "Browse…");
    settle();
    assert_eq!(h.get_takes(), 2);
    std::fs::remove_dir_all(dir).unwrap();
}

fn uploads_show_where_they_stand() {
    let (h, _, _) = harness();
    for text in ["40%", "2.4 MB", "Network lost"] {
        assert_eq!(ElementHandle::find_by_accessible_label(&h, text).count(), 1, "{text}");
    }
    click_label(&h, "Cancel");
    assert_eq!(h.get_acted(), "cancel 0");
    click_label(&h, "Remove");
    assert_eq!(h.get_acted(), "remove 1");
    click_label(&h, "Retry");
    assert_eq!(h.get_acted(), "retry 2");
}

cases!(file_fields: a_press_opens_the_dialog_and_fills_the_field, several_show_the_first_and_a_count_and_clear_empties, drops_tint_and_take_what_fits, a_zone_of_pictures_refuses_the_rest, uploads_show_where_they_stand);
