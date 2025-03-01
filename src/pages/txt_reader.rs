use bevy::{
    prelude::*,
    tasks::{block_on, poll_once, AsyncComputeTaskPool, Task},
    utils::info,
};
use rfd::FileHandle;

use crate::{
    components::{
        button::normal_button::NormalButton,
        viewer::txt::{
            create_txt_viewer, Paragraph, ParagraphRecv, RawTxt, TxtBase, TxtBody, TxtPara,
        },
    },
    resources::page::{PageState, TxtReaderState},
};

#[derive(Component)]
pub struct TxtReader;

#[derive(Component)]
pub struct BackToRootBtn;

impl NormalButton for BackToRootBtn {}

#[derive(Component)]
pub struct OpenFilePickerBtn;

impl NormalButton for OpenFilePickerBtn {}

pub fn txt_ui_base() -> impl Bundle {
    (
        TxtReader,
        Node {
            display: Display::Flex,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
    )
}

#[derive(Component)]
pub struct ReadUITitle;

#[derive(Component)]
pub struct ReaderHint;

pub fn txt_ui_message() -> impl Bundle {
    info!("Renderd Once");
    (
        ReadUITitle,
        Text::new("Text Reader"),
        TextFont {
            font_size: 48.0,
            ..Default::default()
        },
        TextLayout {
            justify: JustifyText::Center,
            ..Default::default()
        },
        TextColor(Color::WHITE),
    )
}

fn txt_ui_usage_message() -> impl Bundle {
    (
        ReaderHint,
        Text::new("Open from local file"),
        TextFont {
            font_size: 16.0,
            ..Default::default()
        },
        TextLayout {
            justify: JustifyText::Center,
            ..Default::default()
        },
        TextColor(Color::srgb(192.0 / 255.0, 192.0 / 255.0, 192.0 / 255.0)),
    )
}

fn txt_ui_btn_open_local_file(parent: &mut ChildBuilder<'_>) {
    parent
        .spawn(OpenFilePickerBtn::spawn_btn(OpenFilePickerBtn))
        .with_child((
            Text::new("open file picker"),
            TextColor(Color::WHITE),
            TextFont {
                font_size: 16.0,
                ..Default::default()
            },
            TextLayout {
                justify: JustifyText::Center,
                ..Default::default()
            },
        ));
}

fn back_root(parent: &mut ChildBuilder<'_>) {
    parent
        .spawn(BackToRootBtn::spawn_btn(BackToRootBtn))
        .with_child((
            Text::new("Root"),
            TextColor(Color::WHITE),
            TextFont {
                font_size: 16.0,
                ..Default::default()
            },
            TextLayout {
                justify: JustifyText::Center,
                ..Default::default()
            },
        ));
}

fn top_banner() -> impl Bundle {
    (Node {
        display: Display::Flex,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Start,
        width: Val::Percent(100.0),
        height: Val::Px(40.0),
        ..Default::default()
    },)
}

#[derive(Component)]
pub struct TxtUIBody;

fn txt_message_body() -> impl Bundle {
    (
        TxtUIBody,
        Node {
            display: Display::Flex,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            row_gap: Val::Px(10.0),
            column_gap: Val::Px(10.0),
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
    )
}

fn txt_messages_with_btn(parent: &mut ChildBuilder<'_>) {
    parent.spawn(txt_ui_message());
    parent.spawn(txt_ui_usage_message());
    txt_ui_btn_open_local_file(parent);
}

pub fn spawn_text_welcome_ui(
    mut commands: Commands,
    txt_ui: Query<Option<Entity>, With<TxtReader>>,
) {
    if txt_ui.is_empty() {
        info!("Creating text Welcome UI");
        commands.spawn(txt_ui_base()).with_children(|parent| {
            parent.spawn(top_banner()).with_children(back_root);
            parent
                .spawn(txt_message_body())
                .with_children(txt_messages_with_btn);
        });
    }
}

pub fn despawn_text_ui(mut commands: Commands, txt_ui: Query<Entity, With<TxtReader>>) {
    if !txt_ui.is_empty() {
        info("Clean Up Txt UI");
        for entity in txt_ui.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn indicates_wait_for_user_selecting(mut title: Query<Option<&mut Text>, With<ReadUITitle>>) {
    title.iter_mut().flatten().for_each(|mut title| {
        *title = "Waiting for User Selection".into();
    });
}

pub fn indicates_wait_for_file_preparation(
    mut title: Query<Option<&mut Text>, With<ReadUITitle>>,
    reader_state: Res<State<TxtReaderState>>,
) {
    match reader_state.get() {
        TxtReaderState::WaitForLoadingFile(ref file_name) => {
            title.iter_mut().flatten().for_each(|mut title| {
                *title = format!("Loading file: {}", &file_name).into();
            });
        }
        _ => unreachable!(),
    }
}

pub fn remove_txt_messages_for_showing_file(
    body: Query<Entity, With<TxtUIBody>>,
    mut commands: Commands,
    children: Query<&Children>,
) {
    body.iter().for_each(|entity| {
        if let Ok(children) = children.get(entity) {
            children.iter().for_each(|child| {
                commands.entity(*child).despawn_recursive();
            });
        }
    });
}

pub fn on_click_back_to_root_btn(
    mut next_page_state: ResMut<NextState<PageState>>,
    mut query: Query<(&Interaction, &BackToRootBtn)>,
) {
    for (interaction, _) in query.iter_mut() {
        if *interaction == Interaction::Pressed {
            next_page_state.set(PageState::WelcomePage);
        }
    }
}
// TODO: Task塞到TxtBody里, 这样就不用再手动的卸除Task
#[derive(Component)]
pub struct FileHandleAysnc(Task<Option<FileHandle>>);

#[derive(Component)]
pub struct RawTxtAsync(Task<RawTxt>);

pub fn on_click_open_local_file(
    interactions: Query<&Interaction, With<OpenFilePickerBtn>>,
    mut command: Commands,
    reader_state: Res<State<TxtReaderState>>,
    mut next_reader_state: ResMut<NextState<TxtReaderState>>,
) {
    interactions
        .iter()
        .for_each(|interaction| match (interaction, reader_state.get()) {
            (Interaction::Pressed, TxtReaderState::Welcome) => {
                next_reader_state.set(TxtReaderState::WaitForUserSelecting);
                let pool = AsyncComputeTaskPool::get();
                let file_handle: Task<Option<FileHandle>> = pool.spawn(async move {
                    let afd = rfd::AsyncFileDialog::new();
                    afd.add_filter("text", &["txt", "md"]).pick_file().await
                });
                command.spawn(FileHandleAysnc(file_handle));
            }
            _ => {}
        });
}

pub fn read_file(
    mut command: Commands,
    mut file_handles: Query<(Entity, &mut FileHandleAysnc)>,
    mut next_reader_state: ResMut<NextState<TxtReaderState>>,
) {
    file_handles.iter_mut().for_each(|(entity, mut task)| {
        if let Some(Some(handle)) = block_on(poll_once(&mut task.0)) {
            info!("Got handle");
            command.entity(entity).despawn();
            next_reader_state.set(TxtReaderState::WaitForLoadingFile(handle.file_name()));
            let pool = AsyncComputeTaskPool::get();
            let raw_txt: Task<RawTxt> = pool.spawn(async move {
                let raw_txt = handle.read().await;
                info!("File read successfully");
                RawTxt {
                    name: handle.file_name().to_string(),
                    raw: String::from_utf8(raw_txt).unwrap(),
                }
            });
            command.spawn(RawTxtAsync(raw_txt));
        }
    })
}

pub fn handle_new_text(
    mut command: Commands,
    mut raw_txt_tasks: Query<(Entity, &mut RawTxtAsync)>,
    mut window: Query<&mut Window>,
    assets: ResMut<AssetServer>,
    body: Query<Entity, With<TxtUIBody>>,
    mut next_reader_state: ResMut<NextState<TxtReaderState>>,
) {
    let font: Handle<Font> = assets.load("fonts/SourceHanSerifCN-VF.ttf");
    raw_txt_tasks.iter_mut().for_each(|(entity, mut task)| {
        if let Some(raw_text) = block_on(poll_once(&mut task.0)) {
            command.entity(entity).despawn_recursive();
            let (sender, receiver) = flume::unbounded::<Paragraph>();
            command.insert_resource(ParagraphRecv(receiver));
            command.run_system_cached(remove_txt_messages_for_showing_file);

            next_reader_state.set(TxtReaderState::PreDisplaying);
            let title = raw_text.name.clone();
            let mut window = window.single_mut();
            window.title = title;

            command.insert_resource(raw_text.clone());
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

            let body_entity = body.single();
            command.entity(body_entity).with_children(|parent| {
                parent
                    .spawn(TxtBase::render(TxtBase))
                    .with_children(|parent| create_txt_viewer(parent, font.clone()));
            });
        }
    })
}

pub fn add_pagegraph(
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
    match block_on(poll_once(&mut paragraph_async)) {
        Some(Ok(pragraph)) => {
            let content_indexes = pragraph.content;
            let raw_slice = &raw_text.raw[content_indexes[0]..content_indexes[1]];
            for body in body_query.iter() {
                if let Some(mut body) = command.get_entity(body) {
                    body.with_children(|parent| {
                        parent.spawn((
                            TxtPara,
                            Node {
                                flex_direction: FlexDirection::Row,
                                padding: UiRect::all(Val::Px(4.0)),
                                width: Val::Auto,
                                height: Val::Auto,
                                ..Default::default()
                            },
                            Text::new(raw_slice),
                            TextFont {
                                font_size: 16.0,
                                font: font.clone(),
                                ..Default::default()
                            },
                        ));
                    });
                    command.run_system_cached(add_pagegraph);
                }
            }
        }
        Some(Err(_)) => {}
        None => {
            info!("Not ready yet")
        }
    }
}
