use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*, winit::WinitSettings};
use redaerok::{app_info, setup_app_control, setup_camera, user_control};

fn main() {
    let exit = App::new()
        .insert_resource(WinitSettings::desktop_app())
        .add_plugins((DefaultPlugins, FrameTimeDiagnosticsPlugin))
        .insert_resource(ClearColor(Color::srgb(1.0, 1.0, 1.0)))
        .add_systems(Startup, (setup_camera, setup_app_control))
        .add_systems(Update, (user_control, app_info).chain())
        .run();
    match exit {
        AppExit::Success => {}
        AppExit::Error(_) => panic!("An error occurred while running the app"),
    }
}
