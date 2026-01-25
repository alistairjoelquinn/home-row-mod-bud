use iced::widget::{Text, text};
use std::collections::HashMap;

struct App {
    name: String,
    screen: Screen,
    keys: HashMap<Key, KeyConfig>,
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum Key {
    A,
    S,
    D,
    F,
    J,
    K,
    L,
    SemiColon,
}

struct KeyConfig {
    modifier: ModifierType,
    tapping_terms: Vec<u8>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            name: String::from("home row mod bud"),
            screen: Screen::KeySelection,
            keys: HashMap::from([
                (
                    Key::A,
                    KeyConfig {
                        modifier: ModifierType::Ctrl,
                        tapping_terms: vec![],
                    },
                ),
                (
                    Key::S,
                    KeyConfig {
                        modifier: ModifierType::Alt,
                        tapping_terms: vec![],
                    },
                ),
                (
                    Key::D,
                    KeyConfig {
                        modifier: ModifierType::Gui,
                        tapping_terms: vec![],
                    },
                ),
                (
                    Key::F,
                    KeyConfig {
                        modifier: ModifierType::Shift,
                        tapping_terms: vec![],
                    },
                ),
                (
                    Key::J,
                    KeyConfig {
                        modifier: ModifierType::Shift,
                        tapping_terms: vec![],
                    },
                ),
                (
                    Key::K,
                    KeyConfig {
                        modifier: ModifierType::Gui,
                        tapping_terms: vec![],
                    },
                ),
                (
                    Key::L,
                    KeyConfig {
                        modifier: ModifierType::Alt,
                        tapping_terms: vec![],
                    },
                ),
                (
                    Key::SemiColon,
                    KeyConfig {
                        modifier: ModifierType::Ctrl,
                        tapping_terms: vec![],
                    },
                ),
            ]),
        }
    }
}

impl App {
    fn update(&mut self, message: Message) {
        println!("{:#?}", message)
    }
    fn view(&self) -> Text<'_> {
        text(&self.name).size(50)
    }
}

#[derive(Debug, Clone)]
enum Message {}

#[derive(Debug, Clone)]
enum ModifierType {
    Shift,
    Ctrl,
    Alt,
    Gui,
}

#[derive(Debug, Clone)]
enum Screen {
    KeySelection,
    TypingTest,
    Results,
}

fn main() -> iced::Result {
    iced::run(App::update, App::view)
}
