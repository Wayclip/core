use crate::models::error::WayclipError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use wayclip_global_hotkey::hotkey::{Code, Modifiers};

// We are basically re-defining the crossterm's structs to be serializable, so this
// struct can be shared across all, cli & gui without any issues.
#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum WayclipKeyCode {
    Char(char),
    F(u8),
    Enter,
    Esc,
    Backspace,
    Tab,
    Up,
    Down,
    Left,
    Right,
    Insert,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
    CapsLock,
    ScrollLock,
    NumLock,
    PrintScreen,
    Pause,
    Null,
}

impl FromStr for WayclipKeyCode {
    type Err = WayclipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 1 {
            return Ok(WayclipKeyCode::Char(s.parse()?));
        }
        if s.starts_with("f") {
            let num = s.strip_prefix("f");
            return Ok(WayclipKeyCode::F(
                num.ok_or_else(|| WayclipError::NotFound("No num in Fkey found".into()))?
                    .parse()?,
            ));
        }
        match s.to_lowercase().as_str() {
            "enter" => Ok(WayclipKeyCode::Enter),
            "esc" | "escape" => Ok(WayclipKeyCode::Esc),
            "backspace" => Ok(WayclipKeyCode::Backspace),
            "tab" => Ok(WayclipKeyCode::Tab),
            "up" => Ok(WayclipKeyCode::Up),
            "down" => Ok(WayclipKeyCode::Down),
            "left" => Ok(WayclipKeyCode::Left),
            "right" => Ok(WayclipKeyCode::Right),
            "insert" => Ok(WayclipKeyCode::Insert),
            "delete" => Ok(WayclipKeyCode::Delete),
            "home" => Ok(WayclipKeyCode::Home),
            "end" => Ok(WayclipKeyCode::End),
            "page_up" | "pgup" => Ok(WayclipKeyCode::PageUp),
            "page_down" | "pgdown" => Ok(WayclipKeyCode::PageDown),
            "caps_lock" | "caps" => Ok(WayclipKeyCode::CapsLock),
            "scroll_lock" | "scroll" => Ok(WayclipKeyCode::ScrollLock),
            "num_lock" | "num" => Ok(WayclipKeyCode::NumLock),
            "print_screen" | "print" => Ok(WayclipKeyCode::PrintScreen),
            "pause" => Ok(WayclipKeyCode::Pause),
            _ => Err(WayclipError::Validation("Invalid character key".into())),
        }
    }
}

impl std::fmt::Display for WayclipKeyCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WayclipKeyCode::Char(c) => write!(f, "{c}"),
            WayclipKeyCode::F(n) => write!(f, "F{n}"),
            WayclipKeyCode::Enter => write!(f, "Return"),
            WayclipKeyCode::Esc => write!(f, "Escape"),
            WayclipKeyCode::Backspace => write!(f, "BackSpace"),
            WayclipKeyCode::Tab => write!(f, "Tab"),
            WayclipKeyCode::Up => write!(f, "Up"),
            WayclipKeyCode::Down => write!(f, "Down"),
            WayclipKeyCode::Left => write!(f, "Left"),
            WayclipKeyCode::Right => write!(f, "Right"),
            WayclipKeyCode::Insert => write!(f, "Insert"),
            WayclipKeyCode::Delete => write!(f, "Delete"),
            WayclipKeyCode::Home => write!(f, "Home"),
            WayclipKeyCode::End => write!(f, "End"),
            WayclipKeyCode::PageUp => write!(f, "Page_Up"),
            WayclipKeyCode::PageDown => write!(f, "Page_Down"),
            WayclipKeyCode::CapsLock => write!(f, "Caps_Lock"),
            WayclipKeyCode::ScrollLock => write!(f, "Scroll_Lock"),
            WayclipKeyCode::NumLock => write!(f, "Num_Lock"),
            WayclipKeyCode::PrintScreen => write!(f, "Print_Screen"),
            WayclipKeyCode::Pause => write!(f, "Pause"),
            WayclipKeyCode::Null => write!(f, ""),
        }
    }
}

// yes i know, i hate this myself, but what can we do
impl From<WayclipKeyCode> for Code {
    fn from(value: WayclipKeyCode) -> Self {
        match value {
            WayclipKeyCode::Char(c) => match c.to_ascii_lowercase() {
                'a' => Code::KeyA,
                'b' => Code::KeyB,
                'c' => Code::KeyC,
                'd' => Code::KeyD,
                'e' => Code::KeyE,
                'f' => Code::KeyF,
                'g' => Code::KeyG,
                'h' => Code::KeyH,
                'i' => Code::KeyI,
                'j' => Code::KeyJ,
                'k' => Code::KeyK,
                'l' => Code::KeyL,
                'm' => Code::KeyM,
                'n' => Code::KeyN,
                'o' => Code::KeyO,
                'p' => Code::KeyP,
                'q' => Code::KeyQ,
                'r' => Code::KeyR,
                's' => Code::KeyS,
                't' => Code::KeyT,
                'u' => Code::KeyU,
                'v' => Code::KeyV,
                'w' => Code::KeyW,
                'x' => Code::KeyX,
                'y' => Code::KeyY,
                'z' => Code::KeyZ,
                '0' => Code::Digit0,
                '1' => Code::Digit1,
                '2' => Code::Digit2,
                '3' => Code::Digit3,
                '4' => Code::Digit4,
                '5' => Code::Digit5,
                '6' => Code::Digit6,
                '7' => Code::Digit7,
                '8' => Code::Digit8,
                '9' => Code::Digit9,
                ' ' => Code::Space,
                '`' => Code::Backquote,
                '\\' => Code::Backslash,
                '[' => Code::BracketLeft,
                ']' => Code::BracketRight,
                ',' => Code::Comma,
                '=' => Code::Equal,
                '-' => Code::Minus,
                '.' => Code::Period,
                '\'' => Code::Quote,
                ';' => Code::Semicolon,
                '/' => Code::Slash,
                _ => Code::Unidentified,
            },
            WayclipKeyCode::F(n) => match n {
                1 => Code::F1,
                2 => Code::F2,
                3 => Code::F3,
                4 => Code::F4,
                5 => Code::F5,
                6 => Code::F6,
                7 => Code::F7,
                8 => Code::F8,
                9 => Code::F9,
                10 => Code::F10,
                11 => Code::F11,
                12 => Code::F12,
                13 => Code::F13,
                14 => Code::F14,
                15 => Code::F15,
                16 => Code::F16,
                17 => Code::F17,
                18 => Code::F18,
                19 => Code::F19,
                20 => Code::F20,
                21 => Code::F21,
                22 => Code::F22,
                23 => Code::F23,
                24 => Code::F24,
                _ => Code::Unidentified,
            },
            WayclipKeyCode::Enter => Code::Enter,
            WayclipKeyCode::Esc => Code::Escape,
            WayclipKeyCode::Backspace => Code::Backspace,
            WayclipKeyCode::Tab => Code::Tab,
            WayclipKeyCode::Up => Code::ArrowUp,
            WayclipKeyCode::Down => Code::ArrowDown,
            WayclipKeyCode::Left => Code::ArrowLeft,
            WayclipKeyCode::Right => Code::ArrowRight,
            WayclipKeyCode::Insert => Code::Insert,
            WayclipKeyCode::Delete => Code::Delete,
            WayclipKeyCode::Home => Code::Home,
            WayclipKeyCode::End => Code::End,
            WayclipKeyCode::PageUp => Code::PageUp,
            WayclipKeyCode::PageDown => Code::PageDown,
            WayclipKeyCode::CapsLock => Code::CapsLock,
            WayclipKeyCode::ScrollLock => Code::ScrollLock,
            WayclipKeyCode::NumLock => Code::NumLock,
            WayclipKeyCode::PrintScreen => Code::PrintScreen,
            WayclipKeyCode::Pause => Code::Pause,
            WayclipKeyCode::Null => Code::Unidentified,
        }
    }
}

/// The combination of keyboard buttons user has to enter to trigger an action
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct WayclipKeyCombo {
    /// There has to be a key code, like 'C'
    pub key_code: WayclipKeyCode,
    /// And a set of modifiers such as 'ALT' + 'WIN'
    pub key_modifiers: WayclipKeyModifiers,
}

impl std::fmt::Display for WayclipKeyCombo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.key_modifiers.is_empty() {
            write!(f, "{}", self.key_code)
        } else {
            write!(f, "{}+{}", self.key_modifiers, self.key_code)
        }
    }
}

impl WayclipKeyCombo {
    /// Method to create a new combo
    pub fn new(key_code: WayclipKeyCode, key_modifiers: WayclipKeyModifiers) -> Self {
        Self {
            key_code,
            key_modifiers,
        }
    }

    /// Method to build the trigger
    pub fn build_trigger(&self) -> String {
        self.to_string()
    }
}

impl FromStr for WayclipKeyCombo {
    type Err = WayclipError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split: Vec<&str> = s.split("+").map(|item| item.trim()).collect();
        if split.len() < 2 {
            return Err(WayclipError::Validation(
                "Minimum a modifier and a key. e.g.: <modifier_1>+<modifier_2>+...+<key>".into(),
            ));
        }

        let key_str = split
            .pop()
            .ok_or_else(|| WayclipError::NotFound("No last element".into()))?;
        let key_code: WayclipKeyCode = key_str.to_lowercase().as_str().parse()?;

        let mut modifiers = WayclipKeyModifiers::empty();

        for modifier in split.iter() {
            match modifier.to_lowercase().as_str() {
                "shift" => modifiers |= WayclipKeyModifiers::SHIFT,
                "ctrl" | "control" => modifiers |= WayclipKeyModifiers::CTRL,
                "alt" => modifiers |= WayclipKeyModifiers::ALT,
                "meta" | "super" | "cmd" | "win" | "logo" => modifiers |= WayclipKeyModifiers::META,
                a => {
                    return Err(WayclipError::Validation(
                        format!("Invalid modifier: {a}").into(),
                    ));
                }
            }
        }

        Ok(WayclipKeyCombo::new(key_code, modifiers))
    }
}

// This bitflags is so that i can do multiple at same time like
// WayclipKeyModifiers::SHIFT | WayclipKeyModifiers::ALT
bitflags::bitflags! {
    #[allow(missing_docs)]
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct WayclipKeyModifiers: u8 {
        #[allow(missing_docs)]
        const SHIFT = 0b0001;
        #[allow(missing_docs)]
        const CTRL = 0b0010;
        #[allow(missing_docs)]
        const ALT = 0b0100;
        #[allow(missing_docs)]
        const META = 0b1000;
    }
}

impl From<WayclipKeyModifiers> for Modifiers {
    fn from(value: WayclipKeyModifiers) -> Self {
        let mut modifiers = Modifiers::empty();

        if value.contains(WayclipKeyModifiers::ALT) {
            modifiers.insert(Modifiers::ALT);
        }

        if value.contains(WayclipKeyModifiers::CTRL) {
            modifiers.insert(Modifiers::CONTROL)
        }

        if value.contains(WayclipKeyModifiers::META) {
            modifiers.insert(Modifiers::META);
        }

        if value.contains(WayclipKeyModifiers::SHIFT) {
            modifiers.insert(Modifiers::SHIFT);
        }

        modifiers
    }
}

impl std::fmt::Display for WayclipKeyModifiers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() {
            return write!(f, "null");
        }

        let mut parts = Vec::new();

        if self.contains(Self::SHIFT) {
            parts.push("SHIFT");
        }
        if self.contains(Self::CTRL) {
            parts.push("CTRL");
        }
        if self.contains(Self::ALT) {
            parts.push("ALT");
        }
        if self.contains(Self::META) {
            parts.push("LOGO");
        }

        write!(f, "{}", parts.join("+"))
    }
}

impl TryFrom<String> for WayclipKeyCombo {
    type Error = WayclipError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl From<WayclipKeyCombo> for String {
    fn from(val: WayclipKeyCombo) -> Self {
        val.to_string()
    }
}
