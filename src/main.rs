use iced::widget::{Text, text};

#[derive(Default)]
struct App {}

#[derive(Debug, Clone)]
enum Message {}

impl App {
    fn update(&mut self, message: Message) {
        println!("{:#?}", message)
    }
    fn view(&self) -> Text<'_> {
        text("hello!").size(50)
    }
}

fn main() -> iced::Result {
    iced::run(App::update, App::view)
}
