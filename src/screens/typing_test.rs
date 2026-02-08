use iced::{
    Alignment::Center,
    Element,
    Length::Fill,
    widget::{button, column, rich_text, span, text},
};

use crate::message::Message;
use crate::types::{Color, Token};

pub fn view(tokens: &[Token]) -> Element<'_, Message> {
    let title = text("Typing Test").size(32);

    let mut spans: Vec<text::Span<'_, ()>> = Vec::new();
    for token in tokens.iter() {
        match token {
            Token::Word(word) => {
                spans.push(span(word.clone()).color(Color::GREY));
            }
            Token::Combo(modifier, letter) => {
                spans.push(span(format!("{}+{}", modifier, letter)).color(Color::GOLD));
            }
        }
        spans.push(span(" "));
    }

    let paragraph = rich_text(spans).size(22).wrapping(text::Wrapping::Word);

    let show_results_button = button(text("Show Results"))
        .padding([10, 20])
        .on_press(Message::ShowResults);

    column![title, paragraph]
        .spacing(30)
        .width(Fill)
        .push(show_results_button)
        .align_x(Center)
        .into()
}
