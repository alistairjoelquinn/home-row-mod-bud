use iced::{Element, Length::Fill, widget::container};

use crate::message::Message;
use crate::screens;
use crate::types::{Key, KeyConfig, ModifierType, Screen};

pub struct App {
    pub screen: Screen,
    pub keys: Vec<KeyConfig>,
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
            Message::StartTest => self.screen = Screen::TypingTest,
            Message::ShowResults => self.screen = Screen::Results,
        }
    }
    pub fn view(&self) -> Element<'_, Message> {
        let content: Element<Message> = match self.screen {
            Screen::KeySelection => screens::key_selection::view(&self.keys),
            Screen::TypingTest => screens::typing_test::view(),
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
