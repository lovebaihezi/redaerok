use bevy::prelude::*;

use crate::{
    components::button::normal_button::{NormalButton, NormalButtonEvent},
    states::page::PageState,
};

pub struct WelcomePlugin;

#[derive(Component)]
pub struct WelcomeUI;

#[derive(Component)]
pub struct JumpTextPageBtn;

impl NormalButton for JumpTextPageBtn {}

#[derive(Component)]
pub struct JumpAIChatPageBtn;

impl NormalButton for JumpAIChatPageBtn {}

fn welcome_ui_base() -> impl Bundle {
    (
        WelcomeUI,
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: Val::Px(16.0),
            column_gap: Val::Px(16.0),
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..Default::default()
        },
    )
}

fn welcome_ui_message() -> impl Bundle {
    (
        Text::new("Welcome To Redaerok!"),
        TextFont {
            font_size: 48.0,
            ..Default::default()
        },
        TextLayout {
            justify: JustifyText::Center,
            ..Default::default()
        },
        TextColor(Color::WHITE),
    )
}

fn welcome_jump_to_txt() -> impl Bundle {
    (
        Node {
            display: Display::Flex,
            width: Val::Px(128.0),
            padding: UiRect::left(Val::Px(16.0)).with_right(Val::Px(16.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        Text::new("Reader"),
        TextColor(Color::WHITE),
        TextFont {
            font_size: 12.0,
            ..Default::default()
        },
        TextLayout {
            justify: JustifyText::Center,
            ..Default::default()
        },
    )
}

fn welcome_jump_to_aichat() -> impl Bundle {
    (
        Node {
            display: Display::Flex,
            width: Val::Px(128.0),
            padding: UiRect::left(Val::Px(16.0)).with_right(Val::Px(16.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        Text::new("AI Chat"),
        TextColor(Color::WHITE),
        TextFont {
            font_size: 12.0,
            ..Default::default()
        },
        TextLayout {
            justify: JustifyText::Center,
            ..Default::default()
        },
    )
}

pub fn spawn_welcome_ui(
    mut commands: Commands,
    welcome_ui: Query<Option<Entity>, With<WelcomeUI>>,
) {
    if welcome_ui.is_empty() {
        commands.spawn(welcome_ui_base()).with_children(|parent| {
            parent.spawn(welcome_ui_message());
            parent
                .spawn(Node {
                    display: Display::Flex,
                    column_gap: Val::Px(16.0),
                    row_gap: Val::Px(16.0),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(NormalButton::spawn_btn(JumpTextPageBtn))
                        .observe(on_click_txt_btn)
                        .with_child(welcome_jump_to_txt());
                    parent
                        .spawn(NormalButton::spawn_btn(JumpAIChatPageBtn))
                        .with_child(welcome_jump_to_aichat());
                });
        });
    }
}

pub fn despawn_welcome_ui(
    mut commands: Commands,
    welcome_ui: Query<Option<Entity>, With<WelcomeUI>>,
) {
    if !welcome_ui.is_empty() {
        welcome_ui.iter().flatten().for_each(|ui_entity| {
            commands.entity(ui_entity).despawn_recursive();
        });
    }
}

pub fn on_click_txt_btn(
    e: Trigger<NormalButtonEvent>,
    mut next_page_state: ResMut<NextState<PageState>>,
) {
    if e.event() == &NormalButtonEvent::Clicked {
        next_page_state.set(PageState::TxtReadPage);
    }
}

impl Plugin for WelcomePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(PageState::WelcomePage), spawn_welcome_ui)
            .add_systems(OnExit(PageState::WelcomePage), despawn_welcome_ui);
    }
}
