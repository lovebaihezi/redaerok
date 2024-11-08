use bevy::prelude::{Camera2dBundle, Commands};

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
