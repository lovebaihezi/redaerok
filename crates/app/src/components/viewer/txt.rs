use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    picking::focus::HoverMap,
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
    utils::futures,
};

use flume::Receiver;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Resource)]
pub struct RawTxt {
    name: String,
    raw: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Paragraph {
    index: usize,
    content: [usize; 2],
}

#[derive(Component)]
pub struct TxtBase;

#[derive(Component)]
pub struct TxtTitle;

#[derive(Component)]
pub struct TxtBody;

#[derive(Component)]
pub struct TxtPara;

#[derive(Resource)]
pub struct ParagraphRecv(Receiver<Paragraph>);

pub fn setup_txt_viewer(
    mut command: Commands,
    txt_base_query: Query<Entity, With<TxtBase>>,
    assests: Res<AssetServer>,
) {
    let font = assests.load("fonts/SourceHanSerifCN-VF.ttf");
    if let Ok(txt_base) = txt_base_query.get_single() {
        if let Some(mut entity_cmd) = command.get_entity(txt_base) {
            entity_cmd
                .with_child((
                    TxtBase,
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
                ))
                .with_children(|parent| {
                    parent
                        .spawn((
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
                        ))
                        .with_child((
                            Text::new("Untitled"),
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
                });
        }
    }
}

#[derive(Component)]
pub struct RawTxtAsync(Task<Option<RawTxt>>);

pub fn pick_file_using_rfs(mut command: Commands) {
    let pool = AsyncComputeTaskPool::get();
    let file_handle: Task<Option<RawTxt>> = pool.spawn(async move {
        let afd = rfd::AsyncFileDialog::new();
        if let Some(file) = afd.add_filter("text", &["txt", "md"]).pick_file().await {
            let file_name = file.file_name();
            let file_content = file.read().await;
            Some(RawTxt {
                name: file_name,
                //TODO(chaibowen): the content may not encode in utf-8, should support it
                raw: String::from_utf8_lossy(&file_content).to_string(),
            })
        } else {
            None
        }
    });
    command.spawn(RawTxtAsync(file_handle));
}

pub fn handle_new_text(mut command: Commands, mut raw_txt_tasks: Query<&mut RawTxtAsync>) {
    for mut task in raw_txt_tasks.iter_mut() {
        if let Some(Some(raw_text)) = futures::check_ready(&mut task.0) {
            command.insert_resource(raw_text.clone());
            let (sender, receiver) = flume::unbounded::<Paragraph>();
            command.insert_resource(ParagraphRecv(receiver));
            let task_pool = AsyncComputeTaskPool::get();
            task_pool
                .spawn(async move {
                    let mut start = 0usize;
                    for (index, line) in raw_text.raw.lines().enumerate() {
                        let paragraph = Paragraph {
                            index,
                            content: [start, start + line.len()],
                        };
                        start += line.len() + 1;
                        sender.send_async(paragraph).await.unwrap();
                    }
                })
                .detach();
        }
    }
}

pub fn update_title_based_on_current_article(
    raw_text: Res<RawTxt>,
    txt_title_query: Query<&Children, With<TxtTitle>>,
    mut text_query: Query<&mut Text>,
    mut window: Query<&mut Window>,
) {
    let mut window = window.single_mut();
    if window.name.as_ref() == Some(&raw_text.name) {
        return;
    }
    for txt_title in &mut txt_title_query.iter() {
        let mut content = text_query.get_mut(txt_title[0]).unwrap();
        **content = raw_text.name.to_string();
    }
    window.name = Some(raw_text.name.to_string());
}

pub fn txt_viewer_render_txt(
    mut channel: ResMut<ParagraphRecv>,
    mut command: Commands,
    raw_text: Res<RawTxt>,
    body_query: Query<Entity, With<TxtBody>>,
    asset_server: ResMut<AssetServer>,
) {
    let font = asset_server.load("fonts/SourceHanSerifCN-VF.ttf");
    let channel = channel.as_mut();
    let rec = channel.0.clone();
    if rec.is_empty() {
        return;
    }
    let mut paragraph_async = rec.recv_async();
    match futures::check_ready(&mut paragraph_async) {
        Some(Ok(pragraph)) => {
            let content_indexes = pragraph.content;
            let raw_slice = &raw_text.raw[content_indexes[0]..content_indexes[1]];
            for body in body_query.iter() {
                if let Some(mut body) = command.get_entity(body) {
                    body.with_children(|parent| {
                        parent
                            .spawn((
                                TxtPara,
                                Node {
                                    flex_direction: FlexDirection::Row,
                                    padding: UiRect::all(Val::Px(4.0)),
                                    width: Val::Auto,
                                    height: Val::Auto,
                                    ..Default::default()
                                },
                            ))
                            .with_child((
                                Text::new(raw_slice),
                                TextFont {
                                    font_size: 16.0,
                                    font: font.clone(),
                                    ..Default::default()
                                },
                            ));
                    });
                    command.run_system_cached(txt_viewer_render_txt);
                }
            }
        }
        Some(Err(_)) => {}
        None => {
            info!("Not ready yet")
        }
    }
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
