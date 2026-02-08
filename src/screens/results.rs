use iced::{
    Alignment::Center,
    Element,
    widget::{button, column, text},
};

use crate::message::Message;

pub fn view() -> Element<'static, Message> {
    let title = text("Results Page").size(32);
    let restart_button = button(text("Try again?"))
        .padding([10, 20])
        .on_press(Message::Restart);
    column![title]
        .spacing(30)
        .push(restart_button)
        .align_x(Center)
        .into()
}
