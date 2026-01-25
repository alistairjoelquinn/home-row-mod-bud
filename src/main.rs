use iced::widget::{Text, text};
use std::collections::HashMap;

struct App {
    name: String,
    screen: Screen,
    keys: HashMap<Key, ModifierType>,
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

impl Default for App {
    fn default() -> Self {
        Self {
            name: String::from("home row mod bud"),
            screen: Screen::KeySelection,
            keys: HashMap::from([
                (Key::A, ModifierType::Shift),
                (Key::S, ModifierType::Ctrl),
                (Key::D, ModifierType::Alt),
                (Key::F, ModifierType::Gui),
                (Key::J, ModifierType::Gui),
                (Key::K, ModifierType::Alt),
                (Key::L, ModifierType::Ctrl),
                (Key::SemiColon, ModifierType::Shift),
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
