use iced::{
    Element,
    Length::Fill,
    widget::{column, container, text},
};
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
    fn view(&self) -> Element<'_, Message> {
        let content: Element<Message> = match self.screen {
            Screen::KeySelection => self.view_key_selection(),
            Screen::TypingTest => self.view_typing_test(),
            Screen::Results => self.view_results(),
        };
        container(content)
            .width(Fill)
            .height(Fill)
            .center_x(Fill)
            .center_y(Fill)
            .padding(40)
            .into()
    }
    fn view_key_selection(&self) -> Element<'_, Message> {
        let title = text("Home Row Mod Bud").size(32);
        column![title].into()
    }
    fn view_typing_test(&self) -> Element<'_, Message> {
        let title = text("Typing Test").size(32);
        column![title].into()
    }
    fn view_results(&self) -> Element<'_, Message> {
        let title = text("Results Page").size(32);
        column![title].into()
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
