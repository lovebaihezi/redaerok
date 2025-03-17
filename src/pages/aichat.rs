use bevy::prelude::*;

use crate::{components::input::normal, states::page::Page};

pub struct AIChatPlugin;

#[derive(Component)]
pub struct AIChatRootInput;

#[derive(Component)]
pub struct AIChatRoot;

fn ai_chat_input_base() -> impl Bundle {
    (
        AIChatRoot,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            border: UiRect::bottom(Val::Px(1.0)),
            ..Default::default()
        },
    )
}

pub fn init_ai_chat_page(mut commands: Commands, assets: ResMut<AssetServer>) {
    let font = assets.load("fonts/SourceHanSerifCN-VF.ttf");
    commands
        .spawn(ai_chat_input_base())
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: Val::Px(256.0),
                        ..Default::default()
                    },
                    BackgroundColor::from(Color::WHITE),
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
        });
}

pub fn remove_aichat_page(mut commands: Commands, query: Query<Entity, With<AIChatRoot>>) {
    for ent in query.iter() {
        commands.entity(ent).despawn_recursive();
    }
}

impl Plugin for AIChatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Page::AIChat), init_ai_chat_page)
            .add_systems(OnExit(Page::AIChat), remove_aichat_page);
    }
}
