use iced::{Element, Length::Fill, widget::container};
use rand::RngExt;
use rand::seq::IndexedRandom;

use crate::message::Message;
use crate::screens;
use crate::types::{Key, KeyConfig, ModifierType, Screen, Token};

const PARAGRAPH: &str = "Alice was beginning to get very tired of sitting by her sister on the bank, and of having nothing to do. Once or twice she had peeped into the book her sister was reading, but it had no pictures or conversations in it, and what is the use of a book, thought Alice, without pictures or conversations? So she was considering in her own mind whether the pleasure of making a daisy chain would be worth the trouble.";

pub struct App {
    pub screen: Screen,
    pub keys: Vec<KeyConfig>,
    pub test_tokens: Vec<Token>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            screen: Screen::KeySelection,
            keys: vec![
                KeyConfig::new(Key::A, ModifierType::Ctrl),
                KeyConfig::new(Key::S, ModifierType::Alt),
                KeyConfig::new(Key::D, ModifierType::Gui),
                KeyConfig::new(Key::F, ModifierType::Shift),
                KeyConfig::new(Key::J, ModifierType::Shift),
                KeyConfig::new(Key::K, ModifierType::Gui),
                KeyConfig::new(Key::L, ModifierType::Alt),
                KeyConfig::new(Key::SemiColon, ModifierType::Ctrl),
            ],
            test_tokens: vec![],
        }
    }
}

impl App {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ModifierSelected(key, modifier) => {
                for item in &mut self.keys {
                    if item.key == key && item.modifier != modifier {
                        item.modifier = modifier;
                    }
                }
            }
            Message::StartTest => {
                self.test_tokens = generate_test_tokens(&self.keys);
                self.screen = Screen::TypingTest;
            }
            Message::ShowResults => self.screen = Screen::Results,
            Message::Restart => self.screen = Screen::KeySelection,
        }
    }
    pub fn view(&self) -> Element<'_, Message> {
        let content: Element<Message> = match self.screen {
            Screen::KeySelection => screens::key_selection::view(&self.keys),
            Screen::TypingTest => screens::typing_test::view(&self.test_tokens),
            Screen::Results => screens::results::view(),
        };
        container(content)
            .width(Fill)
            .height(Fill)
            .center_x(Fill)
            .center_y(Fill)
            .padding(40)
            .into()
    }
}

fn generate_test_tokens(keys: &[KeyConfig]) -> Vec<Token> {
    let mut rng = rand::rng();
    let modifiers: Vec<ModifierType> = keys
        .iter()
        .map(|k| k.modifier)
        .filter(|m| *m != ModifierType::None)
        .collect();
    let letters: Vec<char> = ('a'..='z').collect();

    let words: Vec<&str> = PARAGRAPH.split_whitespace().collect();
    let mut tokens: Vec<Token> = Vec::new();

    for (i, word) in words.iter().enumerate() {
        let word = if rng.random_ratio(1, 5) {
            let mut chars = word.chars();
            match chars.next() {
                Some(c) => c.to_uppercase().to_string() + chars.as_str(),
                None => word.to_string(),
            }
        } else {
            word.to_string()
        };

        tokens.push(Token::Word(word));

        if !modifiers.is_empty() && i > 0 && rng.random_ratio(1, 8) {
            let modifier = *modifiers.choose(&mut rng).unwrap();
            let letter = *letters.choose(&mut rng).unwrap();
            tokens.push(Token::Combo(modifier, letter));
        }
    }
    tokens
}
