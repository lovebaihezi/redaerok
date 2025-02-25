use bevy::{
    dev_tools::fps_overlay::FpsOverlayConfig,
    input::ButtonInput,
    prelude::{KeyCode, Res, ResMut},
};

pub fn setup_game_control(mut overlay: ResMut<FpsOverlayConfig>) {
    overlay.enabled = false;
}

pub fn show_fps_overlay(input: Res<ButtonInput<KeyCode>>, mut overlay: ResMut<FpsOverlayConfig>) {
    if input.just_pressed(KeyCode::F1) {
        overlay.enabled = !overlay.enabled;
    }
}
