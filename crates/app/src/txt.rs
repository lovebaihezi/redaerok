use bevy::{
    color::palettes::basic::*,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
    winit::cursor::CursorIcon,
};

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub enum ImportButtonStatus {
    #[default]
    None,
    Hover,
    Importing,
    ImportSuccess,
    ImportFailed,
}

#[derive(Component)]
pub struct ImportButton {
    pub status: ImportButtonStatus,
}

impl ImportButton {
    pub fn new() -> Self {
        Self {
            status: ImportButtonStatus::None,
        }
    }
}

impl ToString for ImportButtonStatus {
    fn to_string(&self) -> String {
        match self {
            ImportButtonStatus::None => "None",
            ImportButtonStatus::Hover => "Hover",
            ImportButtonStatus::Importing => "Importing",
            ImportButtonStatus::ImportSuccess => "ImportSuccess",
            ImportButtonStatus::ImportFailed => "ImportFailed",
        }
        .to_string()
    }
}

pub fn setup_txt_render_system(mut commands: Commands) {
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(128.0),
                        height: Val::Auto,
                        padding: UiRect::all(Val::Px(4.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON),
                ))
                .with_child((
                    Text::new("Import"),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                ));
        });
}

pub fn button_hover_press_ui_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                **text = "Importing...".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = RED.into();
            }
            Interaction::Hovered => {
                **text = "Import <-".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                **text = "Import".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub fn load_txt_from_local_file_system() {}
