use iced::{
    Alignment::Center,
    Element,
    widget::{button, column, text},
};

use crate::message::Message;

pub fn view() -> Element<'static, Message> {
    let title = text("Typing Test").size(32);
    let show_results_button = button(text("Show Results"))
        .padding([10, 20])
        .on_press(Message::ShowResults);
    column![title]
        .spacing(30)
        .push(show_results_button)
        .align_x(Center)
        .into()
}
