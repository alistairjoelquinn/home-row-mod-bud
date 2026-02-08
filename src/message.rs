use iced::keyboard::Event;

use crate::types::{Key, ModifierType};

#[derive(Debug, Clone)]
pub enum Message {
    ModifierSelected(Key, ModifierType),
    StartTest,
    Restart,
    KeyboardEvent(Event),
}
