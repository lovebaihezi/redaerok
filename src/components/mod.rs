use bevy::prelude::*;
use button::normal_button::normal_button_update;
use input::normal::focus_input;
use viewer::txt::txt_viewer_scroll_viewer;

pub mod button;
pub mod input;
pub mod viewer;

pub struct ComponentPlugin;

impl Plugin for ComponentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (txt_viewer_scroll_viewer, normal_button_update, focus_input),
        );
    }
}
