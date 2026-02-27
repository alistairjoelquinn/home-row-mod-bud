use std::time::Instant;

use iced::keyboard;
use iced::{Element, Length::Fill, Subscription, widget::container};
use rand::RngExt;
use rand::seq::IndexedRandom;

use crate::message::Message;
use crate::screens;
use crate::types::{
    ExpectedInput, Key, KeyConfig, ModifierType, Screen, Token, flatten_tokens,
};

// from "Alice's Adventures in Wonderland - Lewis Carroll (1865)", which is in the public domain
const PARAGRAPH: &str = "Alice was beginning to get very tired of sitting by her sister on the bank, and of having nothing to do. Once or twice she had peeped into the book her sister was reading, but it had no pictures or conversations in it, and what is the use of a book, thought Alice, without pictures or conversations? So she was considering in her own mind whether the pleasure of making a daisy chain would be worth the trouble.";

pub struct App {
    pub screen: Screen,
    pub keys: Vec<KeyConfig>,
    pub test_tokens: Vec<Token>,
    pub expected_inputs: Vec<ExpectedInput>,
    pub current_position: usize,
    pub next_touch: ExpectedInput,
    pub expect_modifier: bool,
    pub timer_start: Option<Instant>,
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
            expected_inputs: vec![],
            current_position: 0,
            next_touch: ExpectedInput::Char('A'),
            expect_modifier: true, // start with `true` as first letter is always a capital
            timer_start: None,
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
                self.expected_inputs = flatten_tokens(&self.test_tokens);
                self.timer_start = None;
                self.screen = Screen::TypingTest;
            }
            Message::Restart => {
                self.current_position = 0;
                self.timer_start = None;
                for key in &mut self.keys {
                    key.tapping_terms.clear();
                }
                self.screen = Screen::KeySelection
            }
            Message::KeyboardEvent(event) => {
                if self.current_position >= self.expected_inputs.len() {
                    return;
                }

                // Start the timer when the modifier key is engaged for an
                // upcoming combo. Accepts both the modifier being held
                // (ModifiersChanged) and the home-row key firing as a character
                // (firmware tap misfire).
                if let ExpectedInput::Combo(modifier_type, _) =
                    &self.expected_inputs[self.current_position]
                {
                    if self.timer_start.is_none() {
                        let modifier_type = *modifier_type;
                        match &event {
                            keyboard::Event::ModifiersChanged(mods) => {
                                let became_active = match modifier_type {
                                    ModifierType::Shift => mods.shift(),
                                    ModifierType::Ctrl => mods.control(),
                                    ModifierType::Alt => mods.alt(),
                                    ModifierType::Gui => mods.logo(),
                                    ModifierType::None => false,
                                };
                                if became_active {
                                    self.timer_start = Some(Instant::now());
                                }
                            }
                            keyboard::Event::KeyPressed { text, .. } => {
                                let home_row_chars: Vec<char> = self
                                    .keys
                                    .iter()
                                    .filter(|k| k.modifier == modifier_type)
                                    .map(|k| k.key.char())
                                    .collect();
                                if text
                                    .as_ref()
                                    .and_then(|t| t.chars().next())
                                    .is_some_and(|c| {
                                        home_row_chars.contains(&c)
                                    })
                                {
                                    self.timer_start = Some(Instant::now());
                                }
                            }
                            _ => {}
                        }
                    }
                }

                if let keyboard::Event::KeyPressed {
                    key,
                    text,
                    modifiers,
                    ..
                } = event
                {
                    let expected = &self.expected_inputs[self.current_position];
                    let matched = match expected {
                        ExpectedInput::Char(c) => text
                            .as_ref()
                            .and_then(|t| t.chars().next())
                            .is_some_and(|t| t == *c),
                        ExpectedInput::Combo(modifier_type, letter) => {
                            let modifier_held = match modifier_type {
                                ModifierType::Shift => modifiers.shift(),
                                ModifierType::Ctrl => modifiers.control(),
                                ModifierType::Alt => modifiers.alt(),
                                ModifierType::Gui => modifiers.logo(),
                                ModifierType::None => false,
                            };
                            let is_match = if let keyboard::Key::Character(k) =
                                &key
                            {
                                k.chars().next().is_some_and(|k| k == *letter)
                            } else {
                                false
                            };
                            modifier_held && is_match
                        }
                    };

                    if matched {
                        // Record the timing if this was a combo and the timer
                        // was running.
                        if let ExpectedInput::Combo(modifier_type, _) = expected
                        {
                            if let Some(start) = self.timer_start.take() {
                                let duration = start.elapsed();
                                let modifier_type = *modifier_type;
                                for key_config in &mut self.keys {
                                    if key_config.modifier == modifier_type {
                                        key_config.tapping_terms.push(duration);
                                    }
                                }
                            }
                        }

                        self.current_position += 1;
                        if self.current_position >= self.expected_inputs.len() {
                            self.screen = Screen::Results;
                        } else {
                            self.next_touch = self.expected_inputs
                                [self.current_position]
                                .clone();
                        }
                    }
                }
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let content: Element<Message> = match self.screen {
            Screen::KeySelection => screens::key_selection::view(&self.keys),
            Screen::TypingTest => screens::typing_test::view(
                &self.expected_inputs,
                self.current_position,
            ),
            Screen::Results => screens::results::view(&self.keys),
        };
        container(content)
            .width(Fill)
            .height(Fill)
            .center_x(Fill)
            .center_y(Fill)
            .padding(40)
            .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        match self.screen {
            Screen::TypingTest => {
                keyboard::listen().map(Message::KeyboardEvent)
            }
            _ => Subscription::none(),
        }
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
        let word = if i == 0 || rng.random_ratio(1, 5) {
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
