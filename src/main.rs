use iced::widget::{Text, text};

struct App {
    name: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            name: String::from("home row mod bud"),
        }
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

impl App {
    fn update(&mut self, message: Message) {
        println!("{:#?}", message)
    }
    fn view(&self) -> Text<'_> {
        text(&self.name).size(50)
    }
}

fn main() -> iced::Result {
    iced::run(App::update, App::view)
}
