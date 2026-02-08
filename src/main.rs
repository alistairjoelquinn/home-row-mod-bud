mod app;
mod message;
mod screens;
mod types;

use app::App;

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title("Home Row Mod Bud")
        .subscription(App::subscription)
        .run()
}
