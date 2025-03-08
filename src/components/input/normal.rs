use bevy::prelude::*;

#[derive(Component)]
pub struct NormalInput {
    current_value: String,
}

impl NormalInput {
    pub fn render() -> impl Bundle {
        ()
    }

    pub fn cur_value(&self) -> &str {
        &self.current_value
    }
}

pub fn update_on_keyboard() {}

pub fn update_on_paste() {}

pub fn change_text_indicator_pos() {}
