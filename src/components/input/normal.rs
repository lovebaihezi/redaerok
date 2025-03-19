use bevy::{prelude::*, window::SystemCursorIcon, winit::cursor::CursorIcon};

#[derive(Component)]
pub struct Input;

pub fn normal_input() -> impl Bundle {
    (
        Input,
        Node {
            width: Val::Px(256.0),
            height: Val::Px(32.0),
            ..Default::default()
        },
        BackgroundColor::from(Color::WHITE),
        BorderColor::from(Color::BLACK),
        TextColor::BLACK,
        Text::new("Content"),
        TextLayout::new(JustifyText::Left, LineBreak::WordOrCharacter),
        TextFont {
            font_size: 32.0,
            ..Default::default()
        },
    )
}

#[derive(Component, Default)]
pub enum InputFocus {
    #[default]
    Normal,
    Disabled,
    Hidden,
    Focused,
}

// Update input bundle focus states
pub fn focus_input(
    mut command: Commands,
    texts: Query<(Entity, &Interaction), (Changed<Interaction>, With<Input>)>,
    window: Single<Entity, With<Window>>,
) {
    let input_cursor: CursorIcon = SystemCursorIcon::Text.into();
    let default_cursor: CursorIcon = SystemCursorIcon::Default.into();
    for (ent, interaction) in texts.iter() {
        match interaction {
            Interaction::Hovered => {
                command
                    .entity(*window)
                    .remove::<CursorIcon>()
                    .insert(input_cursor.clone());
            }
            Interaction::Pressed => {
                info!("Pressed");
            }
            Interaction::None => {
                command
                    .entity(*window)
                    .remove::<CursorIcon>()
                    .insert(default_cursor.clone());
            }
        }
    }
}

pub fn update_on_keyboard() {}

pub fn update_on_paste() {}

pub fn change_text_indicator_pos() {}
