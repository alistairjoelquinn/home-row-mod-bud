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
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModifierType::None => write!(formatter, "-"),
            ModifierType::Shift => write!(formatter, "Shift"),
            ModifierType::Ctrl => write!(formatter, "Ctrl"),
            ModifierType::Alt => write!(formatter, "Alt"),
            ModifierType::Gui => match std::env::consts::OS {
                "macos" => write!(formatter, "Cmd"),
                "windows" => write!(formatter, "Win"),
                _ => write!(formatter, "Super"),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum ExpectedInput {
    Char(char),
    Combo(ModifierType, char),
}

pub fn flatten_tokens(tokens: &[Token]) -> Vec<ExpectedInput> {
    let mut inputs = Vec::new();
    for (i, token) in tokens.iter().enumerate() {
        match token {
            Token::Word(word) => {
                for c in word.chars() {
                    inputs.push(ExpectedInput::Char(c));
                }
                if i < tokens.len() - 1 {
                    inputs.push(ExpectedInput::Char(' '));
                }
            }
            Token::Combo(modifier, letter) => {
                inputs.push(ExpectedInput::Combo(*modifier, *letter));
            }
        }
    }
    inputs
}

pub struct Color;

impl Color {
    pub const GREY: iced::Color = iced::Color::from_rgb(0.4, 0.4, 0.4);
    pub const TYPED: iced::Color = iced::Color::from_rgb(0.85, 0.85, 0.85);
    pub const CURSOR: iced::Color = iced::Color::from_rgb(1.0, 1.0, 1.0);
    pub const BADGE: iced::Color = iced::Color::from_rgb(0.7, 0.7, 0.7);
    pub const BADGE_BORDER: iced::Color = iced::Color::from_rgb(0.45, 0.45, 0.45);
    pub const BADGE_ACTIVE: iced::Color = iced::Color::from_rgb(0.9, 0.75, 0.25);
    pub const BADGE_ACTIVE_BORDER: iced::Color = iced::Color::from_rgb(0.9, 0.75, 0.25);
    pub const BADGE_TYPED: iced::Color = iced::Color::from_rgb(0.55, 0.55, 0.55);
    pub const BADGE_TYPED_BORDER: iced::Color = iced::Color::from_rgb(0.35, 0.35, 0.35);
}

pub enum Screen {
    KeySelection,
    TypingTest,
    Results,
}
