use std::{fmt, time::Duration};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hand {
    Left,
    Right,
}

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

impl Key {
    pub fn char(&self) -> char {
        match self {
            Key::A => 'a',
            Key::S => 's',
            Key::D => 'd',
            Key::F => 'f',
            Key::J => 'j',
            Key::K => 'k',
            Key::L => 'l',
            Key::SemiColon => ';',
        }
    }

    pub fn hand(&self) -> Hand {
        match self {
            Key::A | Key::S | Key::D | Key::F => Hand::Left,
            Key::J | Key::K | Key::L | Key::SemiColon => Hand::Right,
        }
    }
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
    pub tapping_terms: Vec<Duration>,
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
    /// The specific home-row key to use as the modifier, plus the letter.
    Combo(Key, char),
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

#[derive(Debug, Clone)]
pub enum ExpectedInput {
    /// A plain character with no modifier (lowercase, space, punctuation).
    Char(char),
    /// An explicit combo badge: specific home-row key + its modifier + letter.
    Combo(Key, ModifierType, char),
    /// A capital letter whose Shift timing should be attributed to a specific
    /// home-row key. Displayed as the plain uppercase character, not a badge.
    UpperChar(Key, char),
}

/// Returns the typing hand for a character on a standard QWERTY layout.
pub fn typing_hand(c: char) -> Hand {
    const LEFT_HAND: &str = "qwertasdfgzxcvb";
    if LEFT_HAND.contains(c.to_ascii_lowercase()) {
        Hand::Left
    } else {
        Hand::Right
    }
}

pub fn flatten_tokens(
    tokens: &[Token],
    keys: &[KeyConfig],
) -> Vec<ExpectedInput> {
    // Pre-compute the Shift key for each hand so we can assign capitals to
    // the opposite hand's key deterministically.
    let left_shift = keys
        .iter()
        .find(|k| k.modifier == ModifierType::Shift && k.key.hand() == Hand::Left)
        .map(|k| k.key);
    let right_shift = keys
        .iter()
        .find(|k| k.modifier == ModifierType::Shift && k.key.hand() == Hand::Right)
        .map(|k| k.key);

    let mut inputs = Vec::new();
    for (i, token) in tokens.iter().enumerate() {
        match token {
            Token::Word(word) => {
                for c in word.chars() {
                    if c.is_uppercase() {
                        // Capital letter: assign to the Shift key on the
                        // opposite hand from the character being typed.
                        let char_hand = typing_hand(c);
                        let shift_key = match char_hand {
                            Hand::Left => right_shift.or(left_shift),
                            Hand::Right => left_shift.or(right_shift),
                        };
                        match shift_key {
                            Some(key) => {
                                inputs.push(ExpectedInput::UpperChar(key, c))
                            }
                            None => inputs.push(ExpectedInput::Char(c)),
                        }
                    } else {
                        inputs.push(ExpectedInput::Char(c));
                    }
                }
            }
            Token::Combo(key, letter) => {
                let modifier = keys
                    .iter()
                    .find(|k| k.key == *key)
                    .map(|k| k.modifier)
                    .unwrap_or(ModifierType::None);
                inputs.push(ExpectedInput::Combo(*key, modifier, *letter));
            }
        }
        if i < tokens.len() - 1 {
            inputs.push(ExpectedInput::Char(' '));
        }
    }
    inputs
}

pub struct Color;

impl Color {
    pub const TEXT: iced::Color = iced::Color::from_rgb(0.4, 0.4, 0.4);
    pub const TEXT_TYPED: iced::Color = iced::Color::from_rgb(0.85, 0.85, 0.85);
    pub const TEXT_ACTIVE: iced::Color = iced::Color::from_rgb(1.0, 1.0, 0.3);
    pub const BADGE: iced::Color = iced::Color::from_rgb(0.7, 0.7, 0.7);
    pub const BADGE_TYPED: iced::Color =
        iced::Color::from_rgb(0.55, 0.55, 0.55);
    pub const BADGE_ACTIVE: iced::Color = iced::Color::from_rgb(1.0, 1.0, 0.3);
    pub const BADGE_BORDER: iced::Color =
        iced::Color::from_rgb(0.45, 0.45, 0.45);
    pub const BADGE_BORDER_TYPED: iced::Color =
        iced::Color::from_rgb(0.35, 0.35, 0.35);
    pub const BADGE_BORDER_ACTIVE: iced::Color =
        iced::Color::from_rgb(1.0, 1.0, 0.3);
}

pub enum Screen {
    KeySelection,
    TypingTest,
    Results,
}
