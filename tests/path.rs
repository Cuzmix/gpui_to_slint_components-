// PathInput, after ely-core/src/forms/path.rs.
#[macro_use]
mod common;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

use common::*;
use slint::ComponentHandle;

slint::slint! {
    import { PathInput, PathRequest, Paths, TextFields, TextFit } from "ui/index.slint";
    export { PathRequest, Paths, TextFields, TextFit }
    export component Harness inherits Window {
        width: 500px;
        height: 200px;
        in-out property <string> file-text;
        in-out property <string> folder-text: "/keep";
        in-out property <string> handed;
        files := PathInput {
            x: 10px;
            y: 10px;
            width: 400px;
            owner: "file";
            text <=> root.file-text;
        }
        PathInput {
            x: 10px;
            y: 60px;
            width: 400px;
            owner: "folder";
            directories: true;
            fill: false;
            text <=> root.folder-text;
            chosen(path) => { root.handed = path; }
        }
    }
}

type Asked = Rc<RefCell<Vec<(String, bool, String)>>>;

fn harness(answer: Option<&'static str>) -> (Harness, Asked) {
    let h = Harness::new().unwrap();
    ely_slint::wire_fields!(h);
    let asked: Asked = Rc::default();
    let log = asked.clone();
    ely_slint::wire_paths!(h, move |request: PathRequest, reply: ely_slint::paths::Answer| {
        log.borrow_mut().push((request.owner.to_string(), request.directories, request.prompt.to_string()));
        reply.send(answer.map(|path| vec![PathBuf::from(path)]));
    });
    show(&h);
    (h, asked)
}

// Runs what the answer queued for the event loop.
fn settle() {
    slint::invoke_from_event_loop(|| slint::quit_event_loop().unwrap()).unwrap();
    slint::run_event_loop().unwrap();
    advance(0);
}

fn browse_fills_the_field() {
    let (h, asked) = harness(Some("/home/a.txt"));
    // Browse sits at the field's end.
    click(h.window(), 370.0, 26.0);
    settle();
    assert_eq!(asked.borrow().as_slice(), &[("file".to_string(), false, "Choose".to_string())]);
    assert_eq!(h.get_file_text(), "/home/a.txt");
    assert_eq!(h.get_folder_text(), "/keep", "only the asking field takes it");
}

fn a_handler_takes_the_path_instead() {
    let (h, asked) = harness(Some("/home/dir"));
    click(h.window(), 370.0, 76.0);
    settle();
    assert_eq!(asked.borrow()[0].1, true, "folders");
    assert_eq!(h.get_handed(), "/home/dir");
    assert_eq!(h.get_folder_text(), "/keep");
    assert_eq!(h.get_file_text(), "");
}

fn cancelling_keeps_the_text() {
    let (h, asked) = harness(None);
    click(h.window(), 370.0, 76.0);
    settle();
    assert_eq!(asked.borrow().len(), 1);
    assert_eq!(h.get_handed(), "");
    assert_eq!(h.get_folder_text(), "/keep");
}

cases!(path_input: browse_fills_the_field, a_handler_takes_the_path_instead, cancelling_keeps_the_text);
