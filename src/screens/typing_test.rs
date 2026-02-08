use iced::{Element, widget::{column, text}};

use crate::message::Message;

pub fn view() -> Element<'static, Message> {
    let title = text("Typing Test").size(32);
    column![title].into()
}
