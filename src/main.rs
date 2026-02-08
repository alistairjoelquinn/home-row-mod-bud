mod app;
mod message;
mod screens;
mod types;

use app::App;

fn main() -> iced::Result {
    iced::run(App::update, App::view)
}
