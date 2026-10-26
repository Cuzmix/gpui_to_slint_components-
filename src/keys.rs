//! Key caps, ported from Ely-GPUI-Components `ely-core/src/typography/keys.rs` and the keystroke
//! grammar of gpui's `Keystroke::parse_with_os`, so a shortcut reads the same as in GPUI.

use std::fmt;

/// Whose conventions key caps follow. Mirrors the Slint `Platform` enum.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Platform {
    /// macOS.
    Mac,
    /// Windows.
    Windows,
    /// Linux.
    Linux,
}

impl Platform {
    /// The system this build targets.
    pub fn current() -> Self {
        if cfg!(target_os = "macos") {
            Platform::Mac
        } else if cfg!(target_os = "windows") {
            Platform::Windows
        } else {
            Platform::Linux
        }
    }
}

/// Modifiers held with a key.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Modifiers {
    /// Control.
    pub control: bool,
    /// Alt or Option.
    pub alt: bool,
    /// Shift.
    pub shift: bool,
    /// Command on the Mac, Super or Win elsewhere.
    pub platform: bool,
    /// Fn.
    pub function: bool,
}

/// One keystroke: its modifiers and its key, lower case.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Keystroke {
    /// What is held.
    pub modifiers: Modifiers,
    /// The key, such as `s`, `enter` or `f12`.
    pub key: String,
}

/// A keystroke gpui's grammar cannot read.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeystrokeError {
    /// What was given.
    pub keystroke: String,
}

impl fmt::Display for KeystrokeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid keystroke {:?}", self.keystroke)
    }
}

impl std::error::Error for KeystrokeError {}

/// One keystroke in gpui key syntax (`cmd-shift-p`, `secondary-s`), `secondary` read for
/// `platform`: Command on the Mac, Control elsewhere.
pub fn keystroke(source: &str, platform: Platform) -> Result<Keystroke, KeystrokeError> {
    let err = || KeystrokeError { keystroke: source.to_owned() };
    let mut modifiers = Modifiers::default();
    let mut key = None;
    let mut parts = source.split('-').peekable();
    while let Some(part) = parts.next() {
        let is = |name: &str| part.eq_ignore_ascii_case(name);
        if is("ctrl") {
            modifiers.control = true;
        } else if is("alt") {
            modifiers.alt = true;
        } else if is("shift") {
            modifiers.shift = true;
        } else if is("fn") {
            modifiers.function = true;
        } else if is("secondary") {
            if platform == Platform::Mac {
                modifiers.platform = true;
            } else {
                modifiers.control = true;
            }
        } else if is("cmd") || is("super") || is("win") {
            modifiers.platform = true;
        } else if let Some(next) = parts.peek() {
            if next.is_empty() && source.ends_with('-') {
                key = Some("-".to_string());
                break;
            } else if next.len() > 1 && next.starts_with('>') {
                key = Some(part.to_ascii_lowercase());
                parts.next();
            } else {
                return Err(err());
            }
        } else {
            if part.len() == 1 && part.as_bytes()[0].is_ascii_uppercase() {
                modifiers.shift = true;
            }
            key = Some(part.to_ascii_lowercase());
        }
    }
    let key = key.or_else(|| {
        let taken = |held: &mut bool, name: &str| std::mem::take(held).then(|| name.to_string());
        taken(&mut modifiers.shift, "shift")
            .or_else(|| taken(&mut modifiers.control, "control"))
            .or_else(|| taken(&mut modifiers.alt, "alt"))
            .or_else(|| taken(&mut modifiers.platform, "platform"))
            .or_else(|| taken(&mut modifiers.function, "function"))
    });
    Ok(Keystroke { modifiers, key: key.ok_or_else(err)? })
}

/// Caps for one keystroke, modifiers first, in platform order.
pub fn keystroke_labels(stroke: &Keystroke, platform: Platform) -> Vec<String> {
    let m = &stroke.modifiers;
    let mac = platform == Platform::Mac;
    let super_key = if platform == Platform::Windows { "Win" } else { "Super" };
    let order: [(bool, &str, &str); 5] = [
        (m.control, "⌃", "Ctrl"),
        (m.alt, "⌥", "Alt"),
        (m.shift, "⇧", "Shift"),
        (m.platform, "⌘", super_key),
        (m.function, "fn", "Fn"),
    ];
    let mut labels: Vec<String> = order
        .iter()
        .filter(|(held, _, _)| *held)
        .map(|(_, mac_label, label)| if mac { *mac_label } else { *label }.to_string())
        .collect();
    labels.push(key_label(&stroke.key, platform));
    labels
}

/// Cap text for a gpui key name.
pub fn key_label(key: &str, platform: Platform) -> String {
    let mac = platform == Platform::Mac;
    let named = match key {
        "enter" => Some(if mac { "↩" } else { "Enter" }),
        "escape" => Some(if mac { "⎋" } else { "Esc" }),
        "tab" => Some(if mac { "⇥" } else { "Tab" }),
        "backspace" => Some(if mac { "⌫" } else { "Backspace" }),
        "delete" => Some(if mac { "⌦" } else { "Del" }),
        "space" => Some("Space"),
        "up" => Some("↑"),
        "down" => Some("↓"),
        "left" => Some("←"),
        "right" => Some("→"),
        "home" => Some("Home"),
        "end" => Some("End"),
        "pageup" => Some("PgUp"),
        "pagedown" => Some("PgDn"),
        _ => None,
    };
    match named {
        Some(label) => label.to_string(),
        None => key.to_uppercase(),
    }
}

/// A keystroke in gpui key syntax as `platform` spells it: `⌘S`, or `Ctrl+S`.
/// This is what `Button.shortcut` expects.
pub fn shortcut_text(source: &str, platform: Platform) -> Result<String, KeystrokeError> {
    let glue = if platform == Platform::Mac { "" } else { "+" };
    Ok(keystroke_labels(&keystroke(source, platform)?, platform).join(glue))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn labels(source: &str, platform: Platform) -> Vec<String> {
        keystroke_labels(&keystroke(source, platform).unwrap(), platform)
    }

    #[test]
    fn mac_uses_symbols_in_apple_order() {
        assert_eq!(labels("cmd-shift-p", Platform::Mac), ["⇧", "⌘", "P"]);
        assert_eq!(labels("ctrl-alt-enter", Platform::Mac), ["⌃", "⌥", "↩"]);
    }

    #[test]
    fn other_platforms_spell_modifiers() {
        assert_eq!(labels("ctrl-shift-p", Platform::Windows), ["Ctrl", "Shift", "P"]);
        assert_eq!(labels("cmd-escape", Platform::Linux), ["Super", "Esc"]);
    }

    #[test]
    fn secondary_reads_for_the_platform() {
        assert_eq!(labels("secondary-s", Platform::Mac), ["⌘", "S"]);
        assert_eq!(labels("secondary-s", Platform::Windows), ["Ctrl", "S"]);
    }

    #[test]
    fn keys_keep_their_names_or_upper_case() {
        assert_eq!(key_label("f12", Platform::Mac), "F12");
        assert_eq!(key_label("space", Platform::Windows), "Space");
        assert_eq!(key_label("left", Platform::Linux), "←");
    }

    #[test]
    fn shortcut_text_joins_by_platform() {
        assert_eq!(shortcut_text("secondary-s", Platform::Mac).unwrap(), "⌘S");
        assert_eq!(shortcut_text("secondary-s", Platform::Linux).unwrap(), "Ctrl+S");
        assert_eq!(shortcut_text("ctrl--", Platform::Linux).unwrap(), "Ctrl+-");
    }

    #[test]
    fn upper_case_key_means_shift_and_bad_input_fails() {
        assert_eq!(labels("P", Platform::Linux), ["Shift", "P"]);
        assert!(keystroke("ctrl-a-b", Platform::Linux).is_err());
    }
}
