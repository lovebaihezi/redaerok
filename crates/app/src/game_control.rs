use bevy::{
    dev_tools::fps_overlay::FpsOverlayConfig,
    input::ButtonInput,
    prelude::{Commands, KeyCode, Res, ResMut},
    time::{Time, Virtual},
};

pub fn setup_game_control(commands: Commands, mut time: ResMut<Time<Virtual>>) {
    time.pause();
    _ = commands;
}

pub fn show_fps_overlay(input: Res<ButtonInput<KeyCode>>, mut overlay: ResMut<FpsOverlayConfig>) {
    if input.just_pressed(KeyCode::F1) {
        overlay.enabled = !overlay.enabled;
    }
}
