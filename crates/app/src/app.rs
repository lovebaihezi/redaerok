use clap::Parser;
use std::time::Duration;

use crate::{
    camera::normal_camera,
    components, pages,
    resources::AppOptions,
    show_fps_overlay,
    test_functions::{render_to_image_setup, CaptureFramePlugin, ImageCopyPlugin, SceneController},
};

use bevy::{
    app::{PluginGroupBuilder, ScheduleRunnerPlugin},
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
    text::FontSmoothing,
    winit::{WinitPlugin, WinitSettings},
};

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

impl Game {
    pub fn init(app_type: AppType) -> Self {
        #[cfg(target_arch = "wasm32")]
        let options = AppOptions::parse_from(&[""]);
        #[cfg(not(target_arch = "wasm32"))]
        let options = AppOptions::parse();
        let mut game = Game { app: App::new() };
        game.app
            .add_plugins((default_plugins(app_type), fps_plugin()))
            .insert_resource(options)
            .insert_resource(WinitSettings::desktop_app())
            .add_systems(Startup, normal_camera)
            .add_systems(Update, show_fps_overlay);
        match app_type {
            AppType::Normal => {
                game.app
                    .add_systems(Startup, pages::welcome::setup_welcome_ui)
                    .add_systems(
                        FixedUpdate,
                        (
                            components::viewer::txt::handle_new_text
                                .after(components::viewer::txt::setup_txt_viewer),
                            components::viewer::txt::txt_viewer_render_txt,
                            components::viewer::txt::update_title_based_on_current_article,
                        ),
                    )
                    .add_systems(Update, components::viewer::txt::txt_viewer_scroll_viewer);
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
