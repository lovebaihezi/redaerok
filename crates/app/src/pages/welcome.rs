use bevy::prelude::*;

#[derive(Component)]
pub struct WelcomeUI;

fn welcome_ui_base() -> (WelcomeUI, Node) {
    (
        WelcomeUI,
        Node {
            display: Display::Flex,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..Default::default()
        },
    )
}

fn welcome_ui_message() -> (Text, TextFont, TextLayout, TextColor) {
    (
        Text::new("Welcome To Redaerok!"),
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

pub fn setup_welcome_ui(mut commands: Commands) {
    commands.spawn(welcome_ui_base()).with_children(|parent| {
        parent.spawn(welcome_ui_message());
    });
}
