use iced::{
    Alignment::Center,
    Element,
    Length::Fill,
    widget::{Space, column, container, pick_list, row, text},
};
use std::fmt;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
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

struct KeyConfig {
    key: Key,
    modifier: ModifierType,
    tapping_terms: Vec<u8>,
}

#[derive(Debug, Clone)]
enum Message {
    ModifierSelected(Key, ModifierType),
}

#[derive(Debug, Clone, PartialEq)]
enum ModifierType {
    None,
    Shift,
    Ctrl,
    Alt,
    Gui,
}

impl ModifierType {
    const ALL: &[ModifierType] = &[
        ModifierType::None,
        ModifierType::Shift,
        ModifierType::Ctrl,
        ModifierType::Alt,
        ModifierType::Gui,
    ];
}

impl fmt::Display for ModifierType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ModifierType::None => write!(f, "-"),
            ModifierType::Shift => write!(f, "Shift"),
            ModifierType::Ctrl => write!(f, "Ctrl"),
            ModifierType::Alt => write!(f, "Alt"),
            ModifierType::Gui => write!(f, "Gui"),
        }
    }
}

enum Screen {
    KeySelection,
    TypingTest,
    Results,
}

struct App {
    name: String,
    screen: Screen,
    keys: Vec<KeyConfig>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            name: String::from("home row mod bud"),
            screen: Screen::KeySelection,
            keys: vec![
                KeyConfig {
                    key: Key::A,
                    modifier: ModifierType::Ctrl,
                    tapping_terms: vec![],
                },
                KeyConfig {
                    key: Key::S,
                    modifier: ModifierType::Alt,
                    tapping_terms: vec![],
                },
                KeyConfig {
                    key: Key::D,
                    modifier: ModifierType::Gui,
                    tapping_terms: vec![],
                },
                KeyConfig {
                    key: Key::F,
                    modifier: ModifierType::Shift,
                    tapping_terms: vec![],
                },
                KeyConfig {
                    key: Key::J,
                    modifier: ModifierType::Shift,
                    tapping_terms: vec![],
                },
                KeyConfig {
                    key: Key::K,
                    modifier: ModifierType::Gui,
                    tapping_terms: vec![],
                },
                KeyConfig {
                    key: Key::L,
                    modifier: ModifierType::Alt,
                    tapping_terms: vec![],
                },
                KeyConfig {
                    key: Key::SemiColon,
                    modifier: ModifierType::Ctrl,
                    tapping_terms: vec![],
                },
            ],
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
        let subtitle = text("Assign each home row key the modifier you use (leave blank for keys with no modifier):").size(16);
        let mut modifier_row = row![].spacing(20);
        for config in &self.keys {
            modifier_row = modifier_row.push(
                column![
                    text(format!("{}", config.key)).size(34),
                    pick_list(
                        ModifierType::ALL,
                        Some(config.modifier.clone()),
                        move |selected| Message::ModifierSelected(config.key, selected),
                    )
                    .width(80)
                ]
                .align_x(Center)
                .spacing(10),
            )
        }
        column![title, subtitle]
            .push(Space::height(Space::new(), 20))
            .push(modifier_row)
            .spacing(10)
            .align_x(Center)
            .into()
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

fn main() -> iced::Result {
    iced::run(App::update, App::view)
}
