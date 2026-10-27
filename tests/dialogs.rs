// AboutDialog, SplashScreen, CrashReporter, UpdateBanner, UpdateDialog and HostedWindow, after
// ely-core/src/shell/dialogs.rs, crash.rs, update.rs and windows.rs.
#[macro_use]
mod common;
use std::cell::RefCell;
use std::rc::Rc;

use common::*;
use i_slint_backend_testing::{AccessibleRole, ElementHandle};
use slint::{ComponentHandle, ModelRc, SharedString, VecModel};

slint::slint! {
    import { AboutDialog, SplashScreen, CrashReporter, UpdateBanner, UpdateDialog, UpdateState, HostedWindow, WindowChrome, Links, TextFormat, Theme } from "ui/index.slint";
    export { UpdateState, WindowChrome, Links, TextFormat }
    export component Harness inherits Window {
        width: 1800px;
        height: 600px;
        in property <string> description: "A component library for GPUI.";
        in property <bool> splash-known: true;
        in property <bool> sendable: true;
        in property <bool> dismissable: true;
        in property <UpdateState> state: UpdateState.available;
        in property <bool> offers: true;
        in property <[string]> notes;
        out property <bool> details: crash.details;
        out property <int> sends;
        out property <int> dismissals;
        out property <int> copies;
        out property <int> downloads;
        out property <int> restarts;
        out property <int> laters;

        HorizontalLayout {
            alignment: start;
            VerticalLayout {
                width: 360px;
                alignment: start;
                AboutDialog {
                    name: "Ely";
                    version: "0.1.1";
                    description: root.description;
                    links: [{ label: "Website", url: "https://example.com" }];
                    copyright: "© 2026 Ely";
                }
            }
            VerticalLayout {
                width: 480px;
                height: 300px;
                SplashScreen {
                    name: "Nimbus";
                    tagline: "Components for GPUI";
                    status: "Loading fonts…";
                    has-progress: root.splash-known;
                    progress: 0.4;
                }
            }
            crash := CrashReporter {
                width: 400px;
                name: "Ely";
                summary: "The renderer stopped.";
                report: "thread 'main' panicked";
                sendable: root.sendable;
                dismissable: root.dismissable;
                send => { root.sends += 1; }
                dismiss => { root.dismissals += 1; }
                copied => { root.copies += 1; }
            }
        }
        UpdateBanner {
            x: 0;
            y: 520px;
            width: 600px;
            version: "2.4.0";
            state: root.state;
            progress: 0.4;
            offers-download: root.offers;
            offers-restart: root.offers;
            offers-later: root.offers;
            download => { root.downloads += 1; }
            restart => { root.restarts += 1; }
            later => { root.laters += 1; }
        }
        UpdateDialog {
            x: 1300px;
            y: 0;
            width: 400px;
            height: 480px;
            name: "Ely";
            current: "2.3.1";
            version: "2.4.0";
            notes: root.notes;
            state: root.state;
            progress: 0.4;
            offers-download: root.offers;
            offers-restart: root.offers;
            offers-later: root.offers;
        }
    }

    export component Hosted inherits HostedWindow {
        heading: "About Ely";
        window-width: Theme.about-window-width;
        window-height: Theme.about-window-height;
        AboutDialog {
            name: "Ely";
            version: "0.1.1";
        }
    }
}

struct Box_ {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

fn find(h: &impl i_slint_backend_testing::ElementRoot, label: &str) -> Vec<Box_> {
    ElementHandle::find_by_accessible_label(h, label)
        .map(|e| {
            let (at, size) = (e.absolute_position(), e.size());
            Box_ { x: at.x, y: at.y, w: size.width, h: size.height }
        })
        .collect()
}

fn one(h: &impl i_slint_backend_testing::ElementRoot, label: &str) -> Box_ {
    let mut found = find(h, label);
    assert!(!found.is_empty(), "nothing labelled {label:?}");
    found.remove(0)
}

fn press(h: &Harness, label: &str) {
    let b = one(h, label);
    click(h.window(), b.x + b.w / 2.0, b.y + b.h / 2.0);
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    ely_slint::wire_text_format!(h);
    h.set_notes(ModelRc::from(Rc::new(VecModel::from(vec![
        SharedString::from("Faster startup."),
        SharedString::from("New icons."),
    ]))));
    show(&h);
    advance(16);
    h
}

// The About content: the initial on the mark, the name, "Version …", the line, the links and
// the copyright; a link opens its address; no line without a description.
fn the_about_dialog_reads_its_credits() {
    let h = harness();
    let seen = Rc::new(RefCell::new(Vec::new()));
    let log = seen.clone();
    h.global::<Links>().on_open(move |url| log.borrow_mut().push(url.to_string()));
    let mark = one(&h, "E");
    assert_eq!((mark.y, mark.h), (16.0 + (64.0 - mark.h) / 2.0, mark.h), "the mark tops the column");
    let name = one(&h, "Ely");
    assert!(name.y >= 16.0 + 64.0 + 8.0 + 12.0);
    assert!(!find(&h, "Version 0.1.1").is_empty());
    assert!(!find(&h, "© 2026 Ely").is_empty());
    press(&h, "Website");
    assert_eq!(*seen.borrow(), vec!["https://example.com".to_string()]);
    assert!(!find(&h, "A component library for GPUI.").is_empty());
    h.set_description("".into());
    advance(16);
    assert!(find(&h, "A component library for GPUI.").is_empty());
}

// The splash shows the progress it is given, or a sweeping bar without one.
fn the_splash_shows_its_progress() {
    let h = harness();
    assert!(!find(&h, "N").is_empty(), "the initial stands in for a glyph");
    assert!(!find(&h, "Loading fonts…").is_empty());
    let bars = || {
        ElementHandle::find_by_element_type_name(&h, "ProgressBar")
            .filter(|e| e.accessible_role() == Some(AccessibleRole::ProgressIndicator))
            .map(|e| e.accessible_value().unwrap_or_default().to_string())
            .collect::<Vec<_>>()
    };
    assert!(bars().contains(&"40%".to_string()), "{:?}", bars());
    h.set_splash_known(false);
    advance(16);
    assert!(bars().contains(&String::new()), "{:?}", bars());
}

// The report opens and shuts from its link button, copies, and offers Send and Don't send only
// with their handlers.
fn the_crash_reporter_shows_copies_and_sends() {
    let h = harness();
    assert!(!find(&h, "Ely quit unexpectedly").is_empty());
    assert!(!h.get_details());
    press(&h, "Show report");
    assert!(h.get_details());
    advance(300);
    assert!(!find(&h, "Hide report").is_empty());
    assert!(!find(&h, "thread 'main' panicked").is_empty());
    press(&h, "Copy report");
    assert_eq!(h.get_copies(), 1);
    press(&h, "Send report");
    press(&h, "Don't send");
    assert_eq!((h.get_sends(), h.get_dismissals()), (1, 1));
    h.set_sendable(false);
    h.set_dismissable(false);
    advance(16);
    assert!(find(&h, "Send report").is_empty());
    assert!(find(&h, "Don't send").is_empty());
}

// The banner's message follows the state; Download while available, "Restart to update" once
// ready, none while downloading; the x puts it off.
fn the_update_banner_follows_the_state() {
    let h = harness();
    assert!(!find(&h, "Version 2.4.0 is available.").is_empty());
    let download = one(&h, "Download");
    assert!(download.y >= 520.0, "the banner's Download");
    click(h.window(), download.x + 4.0, download.y + 4.0);
    assert_eq!(h.get_downloads(), 1);
    press(&h, "x");
    assert_eq!(h.get_laters(), 1);
    h.set_state(UpdateState::Downloading);
    advance(16);
    assert!(!find(&h, "Downloading 2.4.0… 40%").is_empty());
    assert!(find(&h, "Download").is_empty() && find(&h, "Restart to update").is_empty());
    h.set_state(UpdateState::Ready);
    advance(16);
    assert!(!find(&h, "Version 2.4.0 is ready.").is_empty());
    let restart = find(&h, "Restart to update");
    assert!(restart.iter().any(|b| b.y >= 520.0) && restart.iter().any(|b| b.x >= 1300.0), "the banner's and the dialog's");
    let banner = restart.iter().find(|b| b.y >= 520.0).unwrap();
    click(h.window(), banner.x + 4.0, banner.y + 4.0);
    assert_eq!(h.get_restarts(), 1);
    h.set_offers(false);
    advance(16);
    assert!(find(&h, "Restart to update").is_empty() && find(&h, "x").is_empty());
}

// The dialog's headline, what is installed, the notes as dotted lines, and Later beside the main
// action.
fn the_update_dialog_lists_the_notes() {
    let h = harness();
    assert!(!find(&h, "Ely 2.4.0 is available").is_empty());
    assert!(!find(&h, "You have 2.3.1.").is_empty());
    assert_eq!(find(&h, "·").len(), 2);
    let later = one(&h, "Later");
    let download = find(&h, "Download").into_iter().find(|b| b.x >= 1300.0).unwrap();
    assert_eq!(later.y, download.y);
    assert_eq!(later.x + later.w + 8.0, download.x);
    assert_eq!(download.x + download.w, 1300.0 + 400.0 - 32.0);
    h.set_state(UpdateState::Ready);
    advance(16);
    assert!(!find(&h, "Ely 2.4.0 is ready").is_empty());
}

// A hosted window keeps its size, puts the content under a strip the title bar's height, and
// closes on Escape.
fn a_hosted_window_closes_on_escape() {
    let w = Hosted::new().unwrap();
    ely_slint::wire_text_format!(w);
    let closes = Rc::new(RefCell::new(0));
    let seen = closes.clone();
    w.global::<WindowChrome>().on_close(move || *seen.borrow_mut() += 1);
    show(&w);
    advance(16);
    let size = w.window().size().to_logical(w.window().scale_factor());
    assert_eq!((size.width, size.height), (360.0, 400.0));
    let mark = one(&w, "E");
    assert!((mark.y - (38.0 + 16.0 + (64.0 - mark.h) / 2.0)).abs() < 0.5, "{}", mark.y);
    tap(w.window(), slint::platform::Key::Escape);
    assert_eq!(*closes.borrow(), 1);
}

cases!(dialogs: the_about_dialog_reads_its_credits, the_splash_shows_its_progress, the_crash_reporter_shows_copies_and_sends, the_update_banner_follows_the_state, the_update_dialog_lists_the_notes, a_hosted_window_closes_on_escape);
