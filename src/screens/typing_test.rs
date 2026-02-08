use iced::{
    Alignment::Center,
    Element,
    Length::{Fill, Shrink},
    widget::{column, container, row, text},
};

use crate::message::Message;
use crate::types::{Color, ExpectedInput};

pub fn view(inputs: &[ExpectedInput], current_position: usize) -> Element<'_, Message> {
    let title = text("Typing Test").size(32);

    let mut chunks: Vec<(Vec<Element<'_, Message>>, usize)> = Vec::new();
    let mut current_chunk: Vec<Element<'_, Message>> = Vec::new();
    let mut chunk_len: usize = 0;

    for (i, input) in inputs.iter().enumerate() {
        match input {
            ExpectedInput::Char(c) => {
                let is_current = i == current_position;
                let color = if i < current_position {
                    Color::TEXT_TYPED
                } else if is_current {
                    Color::TEXT_ACTIVE
                } else {
                    Color::TEXT
                };
                let display_char = if *c == ' ' && is_current { '·' } else { *c };
                current_chunk.push(text(display_char.to_string()).size(22).color(color).into());
                chunk_len += 1;

                if *c == ' ' {
                    chunks.push((std::mem::take(&mut current_chunk), chunk_len));
                    chunk_len = 0;
                }
            }
            ExpectedInput::Combo(modifier, letter) => {
                if !current_chunk.is_empty() {
                    chunks.push((std::mem::take(&mut current_chunk), chunk_len));
                    chunk_len = 0;
                }

                let label = format!("{} {}", modifier.symbol(), letter);
                let visual_len = label.len() + 1;

                let (text_color, border_color) = if i < current_position {
                    (Color::BADGE_TYPED, Color::BADGE_BORDER_TYPED)
                } else if i == current_position {
                    (Color::BADGE_ACTIVE, Color::BADGE_BORDER_ACTIVE)
                } else {
                    (Color::BADGE, Color::BADGE_BORDER)
                };

                let badge = container(text(label).size(14).color(text_color))
                    .padding([5, 4])
                    .center_x(Shrink)
                    .center_y(Shrink)
                    .style(move |_theme| container::Style {
                        border: iced::Border {
                            color: border_color,
                            width: 1.0,
                            radius: 4.0.into(),
                        },
                        ..Default::default()
                    });
                chunks.push((vec![badge.into()], visual_len));
            }
        }
    }

    if !current_chunk.is_empty() {
        chunks.push((current_chunk, chunk_len));
    }

    let chars_per_line = 80;
    let mut lines: Vec<Element<Message>> = Vec::new();
    let mut current_line: Vec<Element<Message>> = Vec::new();
    let mut line_char_count = 0;

    for (elements, char_count) in chunks {
        if line_char_count + char_count > chars_per_line && !current_line.is_empty() {
            lines.push(
                row(std::mem::take(&mut current_line))
                    .align_y(Center)
                    .into(),
            );
            line_char_count = 0;
        }
        current_line.extend(elements);
        line_char_count += char_count;
    }
    if !current_line.is_empty() {
        lines.push(row(current_line).align_y(Center).into());
    }

    let paragraph = column(lines).spacing(8);

    column![title, paragraph]
        .spacing(30)
        .width(Fill)
        .align_x(Center)
        .into()
}
