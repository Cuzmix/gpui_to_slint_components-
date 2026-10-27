// ShareButton, after ely-core/src/buttons/share.rs.
#[macro_use]
mod common;
use std::cell::RefCell;
use std::rc::Rc;

use common::*;
use i_slint_backend_testing::{AccessibleRole, ElementHandle};
use slint::ComponentHandle;

slint::slint! {
    import { ShareButton, Sharing } from "ui/index.slint";
    export { Sharing }
    export component Harness inherits Window {
        width: 300px;
        height: 100px;
        out property <bool> post-enabled: post.enabled;
        out property <bool> blank-enabled: blank.enabled;
        post := ShareButton {
            x: 40px;
            y: 20px;
            text: "Ely ships 0.2";
            url: "https://example.com/blog/0-2";
        }
        blank := ShareButton {
            x: 100px;
            y: 20px;
        }
    }
}

fn named(h: &Harness, label: &str) -> usize {
    ElementHandle::find_by_accessible_label(h, label)
        .filter(|e| e.accessible_role() == Some(AccessibleRole::Button))
        .count()
}

// Where the picker is not available it shows disabled, and says why; available, a press asks
// for the picker under the button with its text and link. With nothing to share it stays
// disabled.
fn it_asks_the_host_for_the_picker() {
    let h = Harness::new().unwrap();
    show(&h);
    assert!(!h.get_post_enabled());
    assert_eq!(named(&h, "Sharing needs macOS in this build"), 2);
    let asked = Rc::new(RefCell::new(Vec::new()));
    let log = asked.clone();
    h.global::<Sharing>().on_share(move |text, url, x, y, w, height| {
        log.borrow_mut().push((text.to_string(), url.to_string(), [x, y, w, height]));
    });
    click(h.window(), 54.0, 34.0);
    assert!(asked.borrow().is_empty(), "disabled");
    h.global::<Sharing>().set_available(true);
    assert!(h.get_post_enabled() && !h.get_blank_enabled());
    assert_eq!(named(&h, "Share"), 2);
    click(h.window(), 54.0, 34.0);
    assert_eq!(
        *asked.borrow(),
        vec![("Ely ships 0.2".to_string(), "https://example.com/blog/0-2".to_string(), [40.0, 20.0, 28.0, 28.0])]
    );
    click(h.window(), 114.0, 34.0);
    assert_eq!(asked.borrow().len(), 1);
}

cases!(share: it_asks_the_host_for_the_picker);
