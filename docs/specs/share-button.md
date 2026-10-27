# Spec: ShareButton

Source: `ely-core/src/buttons/share.rs`, `platform/mac.rs` (`share`).

- Props: id, text (optional), url (optional); panics with neither.
- An IconButton: `share-2`, tooltip "Share". On macOS a click shows AppKit's `NSSharingServicePicker` for the text (an NSString) and the link (an NSURL; a link that does not parse logs an error) under the button's box (preferred edge MinY, in the view's bottom-up points), keeping the picker until the next share.
- Elsewhere: disabled, tooltip "Sharing needs macOS in this build".

## Slint mapping
- `ShareButton inherits IconButton { text; url }`. It asks the `Sharing` global: `available` and `share(text, url, x, y, width, height)`.
- `ely_slint::wire_sharing!(app)` (feature `desktop`) sets `available` on macOS and answers `share` with `ely_slint::desktop::share`, which finds the window's AppKit view through winit and flips y to AppKit's.
- With nothing to share it shows disabled (gap G50).
