use bevy::prelude::*;

use crate::{components::button::normal_button::NormalButton, resources::PageState};

#[derive(Component)]
pub struct TxtReader;

#[derive(Component)]
pub struct BackToRootBtn;

impl NormalButton for BackToRootBtn {}

pub fn txt_ui_base() -> impl Bundle {
    (
        TxtReader,
        Node {
            display: Display::Flex,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
    )
}

pub fn txt_ui_message() -> impl Bundle {
    (
        Text::new("Txt Reader"),
        TextFont {
            font_size: 32.0,
            ..Default::default()
        },
        TextLayout {
            justify: JustifyText::Center,
            ..Default::default()
        },
        TextColor(Color::WHITE),
    )
}

pub fn manage_text_ui(
    mut commands: Commands,
    page_state: Res<PageState>,
    txt_ui: Query<Option<Entity>, With<TxtReader>>,
) {
    if *page_state == PageState::TxtReadPage && txt_ui.is_empty() {
        commands.spawn(txt_ui_base()).with_children(|parent| {
            parent.spawn(txt_ui_message());
            parent
                .spawn(Node {
                    display: Display::Flex,
                    column_gap: Val::Px(16.0),
                    row_gap: Val::Px(16.0),
                    ..Default::default()
                })
                .with_children(|parent| {});
        });
    } else if *page_state != PageState::TxtReadPage && !txt_ui.is_empty() {
        txt_ui.iter().flatten().for_each(|entity| {
            commands.entity(entity).despawn_recursive();
        })
    }
}
