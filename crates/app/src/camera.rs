use bevy::prelude::{Camera2d, Commands};

pub fn normal_camera(mut commands: Commands) {
    // Spawn the camera
    commands.spawn(Camera2d);
}
