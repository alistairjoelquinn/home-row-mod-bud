use iced::{
    Alignment::Center,
    Element,
    Length::Fill,
    color,
    widget::{button, column, rich_text, span, text},
};

use crate::message::Message;
use crate::types::Token;

pub fn view(tokens: &[Token]) -> Element<'_, Message> {
    let title = text("Typing Test").size(32);

    let mut spans: Vec<text::Span<'_, ()>> = Vec::new();
    for (i, token) in tokens.iter().enumerate() {
        if i > 0 {
            spans.push(span(" "));
        }
        match token {
            Token::Word(word) => {
                spans.push(span(word.clone()).color(color!(0x666666)));
            }
            Token::Combo(modifier, letter) => {
                spans.push(span(format!("{}+{}", modifier, letter)).color(color!(0xe2b714)));
            }
        }
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
