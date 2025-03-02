use clap::Parser;
use std::time::Duration;

use crate::{
    camera::normal_camera,
    components::{
        button::normal_button::normal_button_update,
        viewer::txt::{txt_viewer_cursor, txt_viewer_scroll_viewer},
    },
    pages::{
        self,
        txt_reader::{despawn_text_ui, indicates_wait_for_file_preparation, spawn_text_welcome_ui},
        welcome::{despawn_welcome_ui, on_click_txt_btn, spawn_welcome_ui},
    },
    resources::AppOptions,
    setup_game_control, show_fps_overlay,
    states::page::{PageState, TxtReaderState},
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
            title: "Redaerok".to_string(),
            canvas: Some("#game".to_string()),
            fit_canvas_to_parent: true,
            ..Default::default()
        }),
        AppType::RenderToImageTesting => None,
    };
    let plugin = DefaultPlugins
        .set(WindowPlugin {
            primary_window,
            ..Default::default()
        })
        .set(AssetPlugin {
            #[cfg(target_arch = "wasm32")]
            meta_check: bevy::asset::AssetMetaCheck::Never,
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
            .init_state::<PageState>()
            .add_sub_state::<TxtReaderState>()
            .add_systems(Startup, (normal_camera, setup_game_control))
            .add_systems(Update, show_fps_overlay);
        match app_type {
            AppType::Normal => {
                game.app
                    // Welcome Page
                    .add_systems(OnEnter(PageState::WelcomePage), spawn_welcome_ui)
                    .add_systems(OnExit(PageState::WelcomePage), despawn_welcome_ui)
                    // Interaction System for Welcome Page
                    .add_systems(
                        Update,
                        (on_click_txt_btn,).run_if(in_state(PageState::WelcomePage)),
                    )
                    // Txt Read Page
                    .add_systems(OnExit(PageState::TxtReadPage), despawn_text_ui)
                    // Txt Reader Page Welcome
                    .add_systems(OnEnter(TxtReaderState::Welcome), spawn_text_welcome_ui)
                    .add_systems(
                        OnTransition::<TxtReaderState> {
                            exited: TxtReaderState::Welcome,
                            entered: TxtReaderState::WaitForLoadingFile,
                        },
                        indicates_wait_for_file_preparation,
                    )
                    .add_systems(
                        Update,
                        (
                            pages::txt_reader::handle_new_text
                                .run_if(in_state(TxtReaderState::WaitForLoadingFile)),
                            pages::txt_reader::add_pagegraph
                                .run_if(in_state(TxtReaderState::PreDisplaying)),
                            (
                                pages::txt_reader::on_click_back_to_root_btn,
                                pages::txt_reader::on_click_open_local_file,
                            )
                                .run_if(in_state(PageState::TxtReadPage)),
                        ),
                    )
                    // Txt Reader Page Wait for File Preparation
                    .add_systems(
                        Update,
                        (
                            txt_viewer_scroll_viewer,
                            normal_button_update,
                            txt_viewer_cursor,
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
