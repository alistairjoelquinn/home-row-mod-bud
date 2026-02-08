use iced::{
    Alignment::Center,
    Element,
    Length::{Fill, Shrink},
    widget::{button, column, container, row, text},
};

use crate::message::Message;
use crate::types::{Color, Token};

pub fn view(tokens: &[Token]) -> Element<'_, Message> {
    let title = text("Typing Test").size(32);

    let mut display_elements: Vec<(Element<'_, Message>, usize)> = Vec::new();

    for token in tokens.iter() {
        match token {
            Token::Word(word) => {
                let len = word.len() + 1;
                display_elements.push((
                    text(format!("{} ", word))
                        .size(22)
                        .color(Color::GREY)
                        .into(),
                    len,
                ));
            }
            Token::Combo(modifier, letter) => {
                let label = format!("{} {}", modifier.symbol(), letter);
                let len = label.len() + 1;
                let badge = container(text(label).size(14).color(Color::BADGE))
                    .padding([5, 4])
                    .center_x(Shrink)
                    .center_y(Shrink)
                    .style(|_theme| container::Style {
                        border: iced::Border {
                            color: Color::BADGE_BORDER,
                            width: 1.0,
                            radius: 4.0.into(),
                        },
                        ..Default::default()
                    });
                display_elements.push((badge.into(), len));
            }
        }
    }

    let chars_per_line = 80;
    let mut lines: Vec<Element<Message>> = Vec::new();
    let mut current_line: Vec<Element<Message>> = Vec::new();
    let mut line_char_count = 0;

    for (elem, char_count) in display_elements {
        line_char_count += char_count;
        current_line.push(elem);

        if line_char_count >= chars_per_line {
            lines.push(
                row(std::mem::take(&mut current_line))
                    .align_y(Center)
                    .spacing(4)
                    .into(),
            );
            line_char_count = 0;
        }
    }
    if !current_line.is_empty() {
        lines.push(row(current_line).align_y(Center).spacing(4).into());
    }

    let paragraph = column(lines).spacing(8);

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
