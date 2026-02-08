use std::fmt;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub enum Key {
    A,
    S,
    D,
    F,
    J,
    K,
    L,
    SemiColon,
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Key::A => write!(f, "A"),
            Key::S => write!(f, "S"),
            Key::D => write!(f, "D"),
            Key::F => write!(f, "F"),
            Key::J => write!(f, "J"),
            Key::K => write!(f, "K"),
            Key::L => write!(f, "L"),
            Key::SemiColon => write!(f, ";"),
        }
    }
}

pub struct KeyConfig {
    pub key: Key,
    pub modifier: ModifierType,
    pub tapping_terms: Vec<u8>,
}

impl KeyConfig {
    pub fn new(key: Key, modifier: ModifierType) -> Self {
        Self {
            key,
            modifier,
            tapping_terms: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub enum Token {
    Word(String),
    Combo(ModifierType, char),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModifierType {
    None,
    Shift,
    Ctrl,
    Alt,
    Gui,
}

impl ModifierType {
    pub const ALL: &[ModifierType] = &[
        ModifierType::None,
        ModifierType::Shift,
        ModifierType::Ctrl,
        ModifierType::Alt,
        ModifierType::Gui,
    ];
    pub fn symbol(&self) -> &'static str {
        match self {
            ModifierType::None => "-",
            ModifierType::Shift => "⇧",
            ModifierType::Ctrl => "⌃",
            ModifierType::Alt => "⌥",
            ModifierType::Gui => "⌘",
        }
    }
}

impl fmt::Display for ModifierType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModifierType::None => write!(f, "-"),
            ModifierType::Shift => write!(f, "Shift"),
            ModifierType::Ctrl => write!(f, "Ctrl"),
            ModifierType::Alt => write!(f, "Alt"),
            ModifierType::Gui => match std::env::consts::OS {
                "macos" => write!(f, "Cmd"),
                "windows" => write!(f, "Win"),
                _ => write!(f, "Super"),
            },
        }
    }
}

pub struct Color;

impl Color {
    pub const GREY: iced::Color = iced::Color::from_rgb(0.4, 0.4, 0.4);
    pub const BADGE: iced::Color = iced::Color::from_rgb(0.7, 0.7, 0.7);
    pub const BADGE_BORDER: iced::Color = iced::Color::from_rgb(0.45, 0.45, 0.45);
}

pub enum Screen {
    KeySelection,
    TypingTest,
    Results,
}
