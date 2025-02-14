use std::time::Duration;

use crate::{
    camera::normal_camera,
    components,
    test_functions::{render_to_image_setup, CaptureFramePlugin, ImageCopyPlugin, SceneController},
};

use bevy::{
    app::{PluginGroupBuilder, ScheduleRunnerPlugin},
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
    text::FontSmoothing,
    winit::WinitPlugin,
};

use clap::Parser;

pub struct Game {
    app: App,
}

/// # AppType: Control App init, plugins and systems
#[derive(Debug, Clone, Copy)]
pub enum AppType {
    Normal,
    RenderToImageTesting,
}

fn default_plugins(app_type: AppType) -> PluginGroupBuilder {
    let primary_window = match app_type {
        AppType::Normal => Some(Window {
            title: "Redaerok(Book Reader)".to_string(),
            canvas: Some("#game".to_string()),
            fit_canvas_to_parent: true,
            ..Default::default()
        }),
        AppType::RenderToImageTesting => None,
    };
    let plugin = DefaultPlugins.set(WindowPlugin {
        primary_window,
        ..Default::default()
    });
    match app_type {
        AppType::RenderToImageTesting => plugin
            .disable::<WinitPlugin>()
            .set(ImagePlugin::default_nearest()),
        AppType::Normal => plugin,
    }
}

fn fps_plugin() -> FpsOverlayPlugin {
    FpsOverlayPlugin {
        config: FpsOverlayConfig {
            text_config: TextFont {
                font_size: 16.0,
                font: default(),
                font_smoothing: FontSmoothing::default(),
            },
            // We can also change color of the overlay
            text_color: Color::linear_rgba(0.0, 1.0, 0.0, 1.0),
            enabled: true,
        },
    }
}

#[derive(Debug, Clone, Parser, Resource)]
#[command(version, about, long_about = None)]
pub struct AppOptions {
    txt_location: Option<String>,
}

impl Game {
    pub fn init(app_type: AppType) -> Self {
        let options = AppOptions::parse();
        let mut game = Game { app: App::new() };
        game.app
            .add_plugins((default_plugins(app_type), fps_plugin()))
            .insert_resource(options)
            .add_systems(Startup, normal_camera);
        match app_type {
            AppType::Normal => {
                game.app
                    .add_systems(Startup, components::viewer::txt::setup_txt_viewer)
                    .add_systems(
                        Update,
                        (
                            components::viewer::txt::txt_viewer_render_txt,
                            components::viewer::txt::txt_viewer_scroll_viewer,
                        ),
                    );
            }
            AppType::RenderToImageTesting => {
                game.app
                    .add_systems(Startup, render_to_image_setup)
                    .add_plugins(ImageCopyPlugin)
                    .add_plugins(CaptureFramePlugin)
                    .add_plugins(ScheduleRunnerPlugin::run_loop(
                        // Run 60 times per second.
                        Duration::from_secs_f64(1.0 / 60.0),
                    ))
                    .init_resource::<SceneController>();
            }
        };
        game
    }

    pub fn run(mut self) -> AppExit {
        self.app.run()
    }
}
