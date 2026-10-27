// EmailInput and UrlInput, after ely-core/src/forms/checked.rs.
#[macro_use]
mod common;
use common::*;
use i_slint_backend_testing::ElementHandle;
use slint::ComponentHandle;

slint::slint! {
    import { EmailInput, UrlInput, TextFields, TextFit } from "ui/index.slint";
    export { TextFields, TextFit }
    export component Harness inherits Window {
        width: 400px;
        height: 200px;
        in-out property <string> email;
        in-out property <string> url;
        out property <bool> email-miss: email.miss;
        out property <bool> url-miss: url.miss;
        public function focus-email() { email.focus(); }
        public function focus-url() { url.focus(); }
        VerticalLayout {
            alignment: start;
            email := EmailInput { text <=> root.email; }
            url := UrlInput { text <=> root.url; }
        }
    }
}

fn harness() -> Harness {
    let h = Harness::new().unwrap();
    ely_slint::wire_fields!(h);
    show(&h);
    h
}

fn type_text(h: &Harness, text: &str) {
    for ch in text.chars() {
        tap(h.window(), ch.to_string());
    }
}

fn notes(h: &Harness) -> usize {
    ElementHandle::find_by_accessible_label(h, "Enter an address like name@example.com").count()
}

fn a_miss_shows_once_focus_leaves() {
    let h = harness();
    h.invoke_focus_email();
    type_text(&h, "ada@example");
    assert!(!h.get_email_miss(), "no verdict while typing");
    assert_eq!(notes(&h), 0);
    h.invoke_focus_url();
    assert!(h.get_email_miss());
    assert_eq!(notes(&h), 1);
    h.invoke_focus_email();
    type_text(&h, ".com");
    h.invoke_focus_url();
    assert!(!h.get_email_miss());
    assert_eq!(notes(&h), 0);
}

fn an_empty_field_never_misses() {
    let h = harness();
    h.invoke_focus_url();
    h.invoke_focus_email();
    assert!(!h.get_url_miss());
}

fn links_need_a_scheme() {
    let h = harness();
    h.set_url("ely.dev".into());
    assert!(h.get_url_miss());
    h.set_url("https://ely.dev".into());
    assert!(!h.get_url_miss());
}

cases!(checked: a_miss_shows_once_focus_leaves, an_empty_field_never_misses, links_need_a_scheme);
