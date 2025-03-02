use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    picking::focus::HoverMap,
    prelude::*,
    window::SystemCursorIcon,
    winit::cursor::CursorIcon,
};

use flume::Receiver;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Paragraph {
    pub index: usize,
    pub content: [usize; 2],
}

#[derive(Component)]
pub struct TxtBase;

impl TxtBase {
    pub fn render(self) -> impl Bundle {
        (
            self,
            Node {
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect::new(
                    Val::Percent(3.0),
                    Val::Percent(3.0),
                    Val::Px(16.0),
                    Val::Px(16.0),
                ),
                ..Default::default()
            },
        )
    }
}

#[derive(Component)]
pub struct TxtTitle;

#[derive(Component)]
pub struct TxtBody;

#[derive(Component)]
pub struct TxtPara;

#[derive(Resource, Clone)]
pub struct RawTxt {
    pub name: String,
    pub raw: String,
}

#[derive(Resource)]
pub struct ParagraphRecv(pub Receiver<Paragraph>);

pub fn create_txt_viewer(parent: &mut ChildBuilder<'_>, font: Handle<Font>, title: String) {
    parent.spawn((
        TxtTitle,
        Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(4.0)),
            border: UiRect::bottom(Val::Px(0.5)),
            overflow: Overflow::scroll_x(),
            ..Default::default()
        },
        BorderColor::from(Color::WHITE),
        Text::new(title),
        TextFont {
            font_size: 24.0,
            font,
            ..Default::default()
        },
        TextLayout {
            justify: JustifyText::Center,
            linebreak: LineBreak::WordOrCharacter,
        },
        TextColor::from(Color::WHITE),
    ));
    parent.spawn((
        TxtBody,
        Node {
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            overflow: Overflow::scroll_y(),
            padding: UiRect::all(Val::Px(4.0)),
            ..Default::default()
        },
        PickingBehavior {
            is_hoverable: true,
            should_block_lower: true,
        },
    ));
}

pub fn txt_viewer_cursor(
    hover_map: Res<HoverMap>,
    mut command: Commands,
    window: Single<Entity, With<Window>>,
    text_query: Query<&Text, Without<Button>>,
    parent_not_button: Query<&Parent, Without<Button>>,
) {
    hover_map.iter().for_each(|(_pointer, pointer_map)| {
        pointer_map.iter().for_each(|(entity, _hit)| {
            // If Parent is not a button, then if it is a text component, set cursor icon to text, else set cursor icon to default
            if parent_not_button.get(*entity).is_ok() {
                if text_query.contains(*entity) {
                    let text_icon: CursorIcon = SystemCursorIcon::Text.into();
                    command
                        .entity(*window)
                        .remove::<CursorIcon>()
                        .insert(text_icon);
                } else {
                    let default_cursor: CursorIcon = SystemCursorIcon::Default.into();
                    command
                        .entity(*window)
                        .remove::<CursorIcon>()
                        .insert(default_cursor);
                }
            } else {
                command.entity(*window).remove::<CursorIcon>();
            }
        })
    })
}

pub fn txt_viewer_scroll_viewer(
    mut scroll_event_reader: EventReader<MouseWheel>,
    hover_map: Res<HoverMap>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    parent: Query<&Parent>,
    mut txt_body_query: Query<&mut ScrollPosition, With<TxtBody>>,
) {
    for event in scroll_event_reader.read() {
        let (mut dx, mut dy) = match event.unit {
            MouseScrollUnit::Line => (event.x * 16.0, event.y * 16.0),
            MouseScrollUnit::Pixel => (event.x, event.y),
        };

        if keyboard_input.pressed(KeyCode::ControlLeft)
            || keyboard_input.pressed(KeyCode::ControlRight)
        {
            std::mem::swap(&mut dx, &mut dy);
        }

        if dy == 0.0 {
            continue;
        }

        for (_pointer, pointer_map) in hover_map.iter() {
            for (entity, _hit) in pointer_map.iter() {
                if let Ok(parent_node) = parent.get(*entity) {
                    if parent.get(**parent_node).is_ok() {
                        for mut scroll in txt_body_query.iter_mut() {
                            scroll.offset_y -= dy * 2.0;
                        }
                    }
                }
            }
        }
    }
}
