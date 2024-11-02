use bevy::{
    color::Color,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    input::ButtonInput,
    prelude::{
        BuildChildren, Commands, Component, KeyCode, MouseButton, NodeBundle, Query, Res, ResMut,
        TextBundle, Touches,
    },
    text::{Text, TextStyle},
    time::{Time, Virtual},
    ui::Style,
    utils::default,
    window::Window,
};

#[derive(Component)]
pub enum Banners {
    FPS,
}

fn base_node() -> NodeBundle {
    NodeBundle {
        style: Style {
            display: bevy::ui::Display::Flex,
            width: bevy::ui::Val::Vw(100.0),
            height: bevy::ui::Val::Vh(100.0),
            align_items: bevy::ui::AlignItems::Center,
            justify_content: bevy::ui::JustifyContent::SpaceBetween,
            flex_direction: bevy::ui::FlexDirection::Column,
            ..default()
        },
        ..default()
    }
}

fn game_info_bundle() -> TextBundle {
    const GAME_VERSION: &str = concat!(
        "game_version: ",
        env!("CARGO_PKG_VERSION"),
        "-",
        env!("GIT_HASH")
    );

    const BUILD_DATE: &str = concat!("build on ", env!("BUILD_DATE"));

    let game_info = format!("{}\n{}", BUILD_DATE, GAME_VERSION);

    TextBundle {
        style: Style {
            align_self: bevy::ui::AlignSelf::Center,
            ..default()
        },
        text: Text::from_section(
            game_info,
            TextStyle {
                color: Color::srgba(0.0, 0.0, 0.0, 1.0),
                font_size: 12.0,
                ..default()
            },
        ),
        ..default()
    }
}

fn branch_boundle() -> TextBundle {
    let branch = env!("GIT_BRANCH");
    TextBundle {
        style: Style {
            align_self: bevy::ui::AlignSelf::Center,
            ..default()
        },
        text: Text::from_section(
            branch,
            TextStyle {
                color: Color::srgba(0.0, 0.0, 0.0, 1.0),
                font_size: 12.0,
                ..default()
            },
        ),
        ..default()
    }
}

fn fps_bundle() -> (TextBundle, Banners) {
    (
        TextBundle {
            style: Style {
                align_self: bevy::ui::AlignSelf::Center,
                ..default()
            },
            text: Text::from_section(
                "ERROR",
                TextStyle {
                    color: Color::srgba(0.0, 0.0, 0.0, 0.96),
                    ..default()
                },
            ),
            ..default()
        },
        Banners::FPS,
    )
}

fn banner() -> NodeBundle {
    NodeBundle {
        style: Style {
            width: bevy::ui::Val::Vw(100.0),
            height: bevy::ui::Val::Auto,
            align_items: bevy::ui::AlignItems::Center,
            justify_content: bevy::ui::JustifyContent::SpaceBetween,
            display: bevy::ui::Display::Flex,
            ..default()
        },
        ..default()
    }
}

pub fn setup_app_control(mut commands: Commands, mut time: ResMut<Time<Virtual>>) {
    time.pause();
    commands.spawn(base_node()).with_children(|parent| {
        parent.spawn(banner()).with_children(|parent| {
            parent.spawn(fps_bundle());
        });
        parent.spawn(banner()).with_children(|parent| {
            parent.spawn(game_info_bundle());
            parent.spawn(branch_boundle());
        });
    });
}

pub fn user_control(
    mut time: ResMut<Time<Virtual>>,
    window: Query<&Window>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    touches: Res<Touches>,
) {
    let window = window.single();
    if window.focused
        && time.is_paused()
        && (keyboard.just_pressed(KeyCode::Space)
            || touches.any_just_pressed()
            || mouse.just_pressed(MouseButton::Left))
    {
        time.unpause();
    } else if !window.focused && !time.is_paused() {
        time.pause();
    };
}

pub fn app_info(mut text_query: Query<(&mut Text, &Banners)>, diagnostics: Res<DiagnosticsStore>) {
    for (mut text, game_control) in text_query.iter_mut() {
        match game_control {
            Banners::FPS => {
                let (fps, avg, smoothed) = diagnostics
                    .get(&FrameTimeDiagnosticsPlugin::FPS)
                    .map(|x| {
                        (
                            x.value().unwrap_or_default(),
                            x.average().unwrap_or_default(),
                            x.smoothed().unwrap_or_default(),
                        )
                    })
                    .unwrap_or_default();
                let fps_info = format!("{fps:.0}|{avg:.0}|{smoothed:.0}");
                text.sections[0].value = fps_info;
            }
        }
    }
}
