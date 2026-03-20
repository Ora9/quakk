use std::fmt::{Debug, Display};

#[derive(Debug)]
pub struct Keypress {
    modifiers: Modifiers,
    key: Key,
}

impl Keypress {
    pub fn from_egui_event(event: egui::Event) -> Option<Self> {
        if let egui::Event::Key {
            key: egui_key,
            modifiers: egui_modifiers,
            pressed: true,
            repeat,
            ..
        } = event
        {
            let key = Key::from_egui_key(egui_key);
            let modifiers = Modifiers::from_egui_modifiers(egui_modifiers);

            if let Some(key) = key {
                Some(Self { key, modifiers })
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn format(&self) -> String {
        let mut s = String::new();

        if !self.modifiers.is_none() {
            s += &self.modifiers.format();
            s += "-";
        }

        s += &self.key.format();

        s
    }
}

impl Display for Keypress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.format())
    }
}

#[derive(Debug)]
pub struct Keybind {
    sequence: Vec<Keypress>,
}

/// The state of the modifiers keys (ctrl, alt and shift) during a keypress
///
/// Shortcomings :
/// - MacOS is currently not handled (command and option)
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Modifiers {
    /// Either of the ctrl ("Control") keys are down
    pub ctrl: bool,

    /// Either the alt keys are down
    pub alt: bool,

    /// Either of the shift keys are down
    pub shift: bool,
}

impl Modifiers {
    const NONE: Self = Self {
        ctrl: false,
        alt: false,
        shift: false,
    };

    const CTRL: Self = Self {
        ctrl: true,
        alt: false,
        shift: false,
    };

    const ALT: Self = Self {
        ctrl: false,
        alt: true,
        shift: false,
    };

    const SHIFT: Self = Self {
        ctrl: false,
        alt: false,
        shift: true,
    };

    /// Get `Modifiers` from [egui modifiers](https://docs.rs/egui/latest/egui/struct.Modifiers.html)
    pub fn from_egui_modifiers(egui_modifiers: egui::Modifiers) -> Self {
        Self {
            ctrl: egui_modifiers.ctrl,
            alt: egui_modifiers.alt,
            shift: egui_modifiers.shift,
        }
    }

    /// Add two modifiers states (OR operation)
    /// ```
    /// # use crate::Modifiers;
    ///
    /// assert_eq!(
    ///     Modifiers::ALT | Modifiers::CTRL,
    ///     Modifiers { ctrl: true, alt: true, shift: false }
    /// );
    pub fn add(&self, rhs: Self) -> Self {
        Self {
            ctrl: self.ctrl | rhs.ctrl,
            alt: self.alt | rhs.alt,
            shift: self.shift | rhs.shift,
        }
    }

    /// Are none of the modifiers keys pressed ?
    pub fn is_none(&self) -> bool {
        self == &Self::NONE
    }

    /// Is any of the modifiers key pressed ?
    pub fn any(&self) -> bool {
        !self.is_none()
    }

    /// Format modifiers state with all pressed modifiers
    /// - in order : `ctrl`, `alt`, `shift`
    /// - in all lowercase
    /// - with `-` (dash) separator
    ///
    /// # Example
    /// - `ctrl`
    /// - `ctrl-shift`
    /// - `alt-shift`
    /// - `ctrl-alt-shift`
    pub fn format(&self) -> String {
        let mut s = String::new();

        let mut append_if = |is_active, name| {
            if is_active {
                if !s.is_empty() {
                    s += "-";
                }
                s += name;
            }
        };

        append_if(self.ctrl, "ctrl");
        append_if(self.alt, "alt");
        append_if(self.shift, "shift");

        s
    }
}

impl std::ops::BitOr for Modifiers {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        self.add(rhs)
    }
}

impl std::ops::BitOrAssign for Modifiers {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl Debug for Modifiers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_none() {
            return write!(f, "Modifiers::NONE");
        }

        let mut debug = f.debug_struct("Modifiers");
        if self.ctrl {
            debug.field("ctrl", &true);
        }
        if self.alt {
            debug.field("alt", &true);
        }
        if self.shift {
            debug.field("shift", &true);
        }
        debug.finish()
    }
}

#[derive(Debug, strum::EnumString, strum::Display)]
#[strum(serialize_all = "lowercase")]
pub enum Key {
    // Commands
    Down,
    Left,
    Right,
    Up,

    Escape,
    Tab,
    Backspace,
    Enter,
    Space,
    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
    Copy,
    Cut,
    Paste,

    // Punctuation
    Colon,
    Comma,
    Backslash,
    Slash,
    Pipe,
    Questionmark,
    Exclamationmark,
    OpenBracket,
    CloseBracket,
    OpenCurlyBracket,
    CloseCurlyBracket,
    Backtick,
    Minus,
    Period,
    Plus,
    Equals,
    Semicolon,
    Quote,

    // Numpad or top row
    #[strum(serialize = "0")]
    Num0,
    #[strum(serialize = "1")]
    Num1,
    #[strum(serialize = "2")]
    Num2,
    #[strum(serialize = "3")]
    Num3,
    #[strum(serialize = "4")]
    Num4,
    #[strum(serialize = "5")]
    Num5,
    #[strum(serialize = "6")]
    Num6,
    #[strum(serialize = "7")]
    Num7,
    #[strum(serialize = "8")]
    Num8,
    #[strum(serialize = "9")]
    Num9,

    // Letters
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    // Function keys
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    F26,
    F27,
    F28,
    F29,
    F30,
    F31,
    F32,
    F33,
    F34,
    F35,
}

impl Key {
    pub fn from_egui_key(egui_key: egui::Key) -> Option<Key> {
        match egui_key {
            // Commands
            egui::Key::ArrowDown => Some(Key::Down),
            egui::Key::ArrowLeft => Some(Key::Left),
            egui::Key::ArrowRight => Some(Key::Right),
            egui::Key::ArrowUp => Some(Key::Up),
            egui::Key::Escape => Some(Key::Escape),
            egui::Key::Tab => Some(Key::Tab),
            egui::Key::Backspace => Some(Key::Backspace),
            egui::Key::Enter => Some(Key::Enter),
            egui::Key::Space => Some(Key::Space),
            egui::Key::Insert => Some(Key::Insert),
            egui::Key::Delete => Some(Key::Delete),
            egui::Key::Home => Some(Key::Home),
            egui::Key::End => Some(Key::End),
            egui::Key::PageUp => Some(Key::PageUp),
            egui::Key::PageDown => Some(Key::PageDown),
            egui::Key::Copy => Some(Key::Copy),
            egui::Key::Cut => Some(Key::Cut),
            egui::Key::Paste => Some(Key::Paste),

            // Punctuation
            egui::Key::Colon => Some(Key::Colon),
            egui::Key::Comma => Some(Key::Comma),
            egui::Key::Backslash => Some(Key::Backslash),
            egui::Key::Slash => Some(Key::Slash),
            egui::Key::Pipe => Some(Key::Pipe),
            egui::Key::Questionmark => Some(Key::Questionmark),
            egui::Key::Exclamationmark => Some(Key::Exclamationmark),
            egui::Key::OpenBracket => Some(Key::OpenBracket),
            egui::Key::CloseBracket => Some(Key::CloseBracket),
            egui::Key::OpenCurlyBracket => Some(Key::OpenCurlyBracket),
            egui::Key::CloseCurlyBracket => Some(Key::CloseCurlyBracket),
            egui::Key::Backtick => Some(Key::Backtick),
            egui::Key::Minus => Some(Key::Minus),
            egui::Key::Period => Some(Key::Period),
            egui::Key::Plus => Some(Key::Plus),
            egui::Key::Equals => Some(Key::Equals),
            egui::Key::Semicolon => Some(Key::Semicolon),
            egui::Key::Quote => Some(Key::Quote),

            // Digits
            egui::Key::Num0 => Some(Key::Num0),
            egui::Key::Num1 => Some(Key::Num1),
            egui::Key::Num2 => Some(Key::Num2),
            egui::Key::Num3 => Some(Key::Num3),
            egui::Key::Num4 => Some(Key::Num4),
            egui::Key::Num5 => Some(Key::Num5),
            egui::Key::Num6 => Some(Key::Num6),
            egui::Key::Num7 => Some(Key::Num7),
            egui::Key::Num8 => Some(Key::Num8),
            egui::Key::Num9 => Some(Key::Num9),

            // Letters
            egui::Key::A => Some(Key::A),
            egui::Key::B => Some(Key::B),
            egui::Key::C => Some(Key::C),
            egui::Key::D => Some(Key::D),
            egui::Key::E => Some(Key::E),
            egui::Key::F => Some(Key::F),
            egui::Key::G => Some(Key::G),
            egui::Key::H => Some(Key::H),
            egui::Key::I => Some(Key::I),
            egui::Key::J => Some(Key::J),
            egui::Key::K => Some(Key::K),
            egui::Key::L => Some(Key::L),
            egui::Key::M => Some(Key::M),
            egui::Key::N => Some(Key::N),
            egui::Key::O => Some(Key::O),
            egui::Key::P => Some(Key::P),
            egui::Key::Q => Some(Key::Q),
            egui::Key::R => Some(Key::R),
            egui::Key::S => Some(Key::S),
            egui::Key::T => Some(Key::T),
            egui::Key::U => Some(Key::U),
            egui::Key::V => Some(Key::V),
            egui::Key::W => Some(Key::W),
            egui::Key::X => Some(Key::X),
            egui::Key::Y => Some(Key::Y),
            egui::Key::Z => Some(Key::Z),

            // Function key
            egui::Key::F1 => Some(Key::F1),
            egui::Key::F2 => Some(Key::F2),
            egui::Key::F3 => Some(Key::F3),
            egui::Key::F4 => Some(Key::F4),
            egui::Key::F5 => Some(Key::F5),
            egui::Key::F6 => Some(Key::F6),
            egui::Key::F7 => Some(Key::F7),
            egui::Key::F8 => Some(Key::F8),
            egui::Key::F9 => Some(Key::F9),
            egui::Key::F10 => Some(Key::F10),
            egui::Key::F11 => Some(Key::F11),
            egui::Key::F12 => Some(Key::F12),
            egui::Key::F13 => Some(Key::F13),
            egui::Key::F14 => Some(Key::F14),
            egui::Key::F15 => Some(Key::F15),
            egui::Key::F16 => Some(Key::F16),
            egui::Key::F17 => Some(Key::F17),
            egui::Key::F18 => Some(Key::F18),
            egui::Key::F19 => Some(Key::F19),
            egui::Key::F20 => Some(Key::F20),
            egui::Key::F21 => Some(Key::F21),
            egui::Key::F22 => Some(Key::F22),
            egui::Key::F23 => Some(Key::F23),
            egui::Key::F24 => Some(Key::F24),
            egui::Key::F25 => Some(Key::F25),
            egui::Key::F26 => Some(Key::F26),
            egui::Key::F27 => Some(Key::F27),
            egui::Key::F28 => Some(Key::F28),
            egui::Key::F29 => Some(Key::F29),
            egui::Key::F30 => Some(Key::F30),
            egui::Key::F31 => Some(Key::F31),
            egui::Key::F32 => Some(Key::F32),
            egui::Key::F33 => Some(Key::F33),
            egui::Key::F34 => Some(Key::F34),
            egui::Key::F35 => Some(Key::F35),
            _ => None,
        }
    }

    pub fn format(&self) -> String {
        self.to_string()
    }
}
