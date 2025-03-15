use bevy::prelude::*;

#[derive(Component)]
pub struct Input;

#[derive(Component)]
pub enum InputFocus {
    Normal,
    Disabled,
    Hidden,
    Focused,
}

pub fn bundle(extra_marker: impl Component, font: TextFont, layout: TextLayout) -> impl Bundle {
    (
        extra_marker,
        Input,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..Default::default()
        },
        Text::new(""),
        font,
        layout,
    )
}

// Update input bundle focus states
pub fn focus_input(texts: Query<(Entity, &Interaction), (Changed<Interaction>, With<Input>)>) {
    for (ent, interaction) in texts.iter() {
        match interaction {
            Interaction::Hovered => {
                // Handle hover state
            }
            Interaction::Pressed => {
                info!("Input Got Focused: {:?}", ent);
            }
            Interaction::None => {
                // Handle none state
            }
        }
    }
}

pub fn update_on_keyboard() {}

pub fn update_on_paste() {}

pub fn change_text_indicator_pos() {}
