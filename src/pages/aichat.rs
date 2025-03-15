use bevy::prelude::*;

use crate::{components::input::normal, states::page::PageState};

pub struct AIChatPlugin;

#[derive(Component)]
pub struct AIChatRootInput;

fn ai_chat_input_base() -> impl Bundle {
    (Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        border: UiRect::bottom(Val::Px(1.0)),
        ..Default::default()
    },)
}

pub fn init_ai_chat_page(mut commands: Commands, assets: ResMut<AssetServer>) {
    let font = assets.load("fonts/SourceHanSerifCN-VF.ttf");
    commands
        .spawn(ai_chat_input_base())
        .with_child((
            Node {
                width: Val::Px(128.0),
                ..Default::default()
            },
            BorderColor::from(Color::BLACK),
        ))
        .with_child(normal::bundle(
            AIChatRootInput,
            TextFont {
                font,
                font_size: 20.0,
                ..Default::default()
            },
            TextLayout::new(JustifyText::Left, LineBreak::WordOrCharacter),
        ));
}

impl Plugin for AIChatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(PageState::AIChatPage), init_ai_chat_page);
    }
}
