use iced::{
    Alignment::{Center, Start},
    Background, Border, Element,
    Length::Fill,
    widget::{button, column, container, row, text},
};

use crate::message::Message;
use crate::types::{Color, KeyConfig, ModifierType};

pub fn view(keys: &[KeyConfig]) -> Element<'_, Message> {
    let title = text("Results").size(32);
    let subtitle = text("Tapping term timings per home row mod key")
        .size(16)
        .color(Color::TEXT);

    let active_keys: Vec<&KeyConfig> = keys
        .iter()
        .filter(|k| k.modifier != ModifierType::None)
        .collect();

    let mut cards = row![].spacing(16).align_y(Start);
    for config in active_keys {
        cards = cards.push(key_card(config));
    }

    let restart_button = button(text("Try again?"))
        .padding([10, 20])
        .on_press(Message::Restart);

    column![title, subtitle, cards, restart_button]
        .spacing(30)
        .align_x(Center)
        .into()
}

fn key_card(config: &KeyConfig) -> Element<'_, Message> {
    let key_label = text(format!("{}", config.key))
        .size(34)
        .color(Color::TEXT_TYPED);

    let modifier_badge = container(
        text(format!("{} {}", config.modifier.symbol(), config.modifier))
            .size(13)
            .color(Color::BADGE),
    )
    .padding([4, 8])
    .style(|_theme| container::Style {
        border: Border {
            color: Color::BADGE_BORDER,
            width: 1.0,
            radius: 4.0.into(),
        },
        ..Default::default()
    });

    // Bordered card contains only the key label and modifier badge —
    // same height for every key regardless of how many timings it has.
    let header = container(
        column![key_label, modifier_badge]
            .spacing(8)
            .align_x(Center),
    )
    .padding([16, 20])
    .width(130)
    .style(|_theme| container::Style {
        border: Border {
            color: Color::BADGE_BORDER,
            width: 1.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    });

    let mut card_col = column![header].spacing(12).align_x(Center);

    if !config.tapping_terms.is_empty() {
        let avg_ms = {
            let total: f64 = config
                .tapping_terms
                .iter()
                .map(|d| d.as_micros() as f64 / 1000.0)
                .sum();
            total / config.tapping_terms.len() as f64
        };

        let mut timings_col = column![].spacing(4).align_x(Center);
        for duration in &config.tapping_terms {
            let ms = duration.as_micros() as f64 / 1000.0;
            timings_col = timings_col.push(
                text(format!("{:.1} ms", ms))
                    .size(14)
                    .color(Color::TEXT_TYPED),
            );
        }

        let separator =
            container(text("")).width(Fill).height(1).style(|_theme| {
                container::Style {
                    background: Some(Background::Color(Color::BADGE_BORDER)),
                    ..Default::default()
                }
            });

        let avg_label = text(format!("avg  {:.1} ms", avg_ms))
            .size(14)
            .color(Color::TEXT_ACTIVE);

        card_col = card_col.push(
            column![timings_col, separator, avg_label]
                .spacing(8)
                .align_x(Center),
        );
    }

    card_col.into()
}
