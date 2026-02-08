use iced::{
    Alignment::Center,
    Element,
    widget::{Space, button, column, pick_list, row, text},
};

use crate::message::Message;
use crate::types::{KeyConfig, ModifierType};

pub fn view(keys: &[KeyConfig]) -> Element<'_, Message> {
    let title = text("Home Row Mod Bud").size(32);
    let subtitle = text(
        "Assign each home row key the modifier you use (leave blank for keys with no modifier):",
    )
    .size(16);
    let mut modifier_row = row![].spacing(20);
    for config in keys {
        modifier_row = modifier_row.push(
            column![
                text(format!("{}", config.key)).size(34),
                pick_list(
                    ModifierType::ALL,
                    Some(config.modifier.clone()),
                    move |modifier| Message::ModifierSelected(config.key, modifier),
                )
                .width(80)
            ]
            .align_x(Center)
            .spacing(10),
        )
    }
    let start_button = button(text("Start Typing Test"))
        .padding([10, 20])
        .on_press(Message::StartTest);
    column![title, subtitle]
        .push(Space::height(Space::new(), 10))
        .push(modifier_row)
        .spacing(30)
        .push(start_button)
        .align_x(Center)
        .into()
}
