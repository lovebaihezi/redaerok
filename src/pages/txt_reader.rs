use bevy::prelude::*;

use crate::{components::button::normal_button::NormalButton, resources::PageState};

#[derive(Component)]
pub struct TxtReader;

#[derive(Component)]
pub struct BackToRootBtn;

impl NormalButton for BackToRootBtn {}

#[derive(Component)]
pub struct OpenFilePickerBtn;

impl NormalButton for OpenFilePickerBtn {}

pub fn txt_ui_base() -> impl Bundle {
    (
        TxtReader,
        Node {
            display: Display::Flex,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
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

fn txt_ui_usage_message() -> impl Bundle {
    (
        Text::new("Open from local file"),
        TextFont {
            font_size: 16.0,
            ..Default::default()
        },
        TextLayout {
            justify: JustifyText::Center,
            ..Default::default()
        },
        TextColor(Color::srgb(192.0 / 255.0, 192.0 / 255.0, 192.0 / 255.0)),
    )
}

fn txt_ui_btn_open_local_file() -> impl Bundle {}

pub fn manage_text_ui(
    mut commands: Commands,
    page_state: Res<PageState>,
    txt_ui: Query<Option<Entity>, With<TxtReader>>,
) {
    if *page_state == PageState::TxtReadPage && txt_ui.is_empty() {
        commands.spawn(txt_ui_base()).with_children(|parent| {
            parent
                .spawn((
                    Node {
                        display: Display::Flex,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Start,
                        width: Val::Percent(100.0),
                        height: Val::Px(40.0),
                        border: UiRect::bottom(Val::Px(1.0)),
                        ..Default::default()
                    },
                    BorderColor::from(Color::WHITE),
                ))
                .with_children(|parent| {
                    parent
                        .spawn(BackToRootBtn::spawn_btn(BackToRootBtn))
                        .with_child((
                            Text::new("Root"),
                            TextColor(Color::WHITE),
                            TextFont {
                                font_size: 16.0,
                                ..Default::default()
                            },
                            TextLayout {
                                justify: JustifyText::Center,
                                ..Default::default()
                            },
                        ));
                });
            parent
                .spawn(Node {
                    display: Display::Flex,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(txt_ui_message());
                    parent.spawn(txt_ui_usage_message());
                    parent
                        .spawn(Node {
                            display: Display::Flex,
                            column_gap: Val::Px(16.0),
                            row_gap: Val::Px(16.0),
                            ..Default::default()
                        })
                        .with_children(|parent| {});
                });
        });
    } else if *page_state != PageState::TxtReadPage && !txt_ui.is_empty() {
        txt_ui.iter().flatten().for_each(|entity| {
            commands.entity(entity).despawn_recursive();
        })
    }
}

pub fn on_click_back_to_root_btn(
    mut page_state: ResMut<PageState>,
    mut query: Query<(&Interaction, &BackToRootBtn)>,
) {
    for (interaction, _) in query.iter_mut() {
        if *interaction == Interaction::Pressed {
            *page_state = PageState::WelcomePage;
        }
    }
}
