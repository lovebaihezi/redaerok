use bevy::{
    prelude::*,
    tasks::{block_on, poll_once, AsyncComputeTaskPool, Task},
    utils::info,
};

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
    info!("Rendered Once");
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

// TODO: 不在State里存title, 应当是生成到TxtBase Component再读取
pub fn indicates_wait_for_file_preparation(mut title: Query<Option<&mut Text>, With<ReadUITitle>>) {
    title.iter_mut().flatten().for_each(|mut title| {
        *title = "Loading file".to_string().into();
    });
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
    mut query: Query<&Interaction, (With<BackToRootBtn>, Changed<Interaction>)>,
) {
    for interaction in query.iter_mut() {
        if *interaction == Interaction::Pressed {
            next_page_state.set(PageState::WelcomePage);
        }
    }
}
// TODO: Task塞到TxtBody里, 这样就不用再手动的卸除Task

#[derive(Component)]
pub struct RawTxtAsync(Task<Option<RawTxt>>);

pub fn on_click_open_local_file(
    interactions: Query<&Interaction, (Changed<Interaction>, With<OpenFilePickerBtn>)>,
    mut command: Commands,
    reader_state: Res<State<TxtReaderState>>,
    mut next_reader_state: ResMut<NextState<TxtReaderState>>,
) {
    interactions.iter().for_each(|interaction| {
        if let (Interaction::Pressed, TxtReaderState::Welcome) = (interaction, reader_state.get()) {
            next_reader_state.set(TxtReaderState::WaitForLoadingFile);
            let pool = AsyncComputeTaskPool::get();
            let file_handle: Task<Option<_>> = pool.spawn(async move {
                let afd = rfd::AsyncFileDialog::new();
                if let Some(handle) = afd.add_filter("text", &["txt", "md"]).pick_file().await {
                    let raw_txt = handle.read().await;
                    info!("File read successfully");
                    Some(RawTxt {
                        name: handle.file_name().to_string(),
                        raw: String::from_utf8(raw_txt).unwrap(),
                    })
                } else {
                    None
                }
            });
            command.spawn(RawTxtAsync(file_handle));
        }
    });
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
        if let Some(Some(raw_text)) = block_on(poll_once(&mut task.0)) {
            command.entity(entity).despawn_recursive();
            let (sender, receiver) = flume::unbounded::<Paragraph>();
            command.insert_resource(ParagraphRecv(receiver));
            command.run_system_cached(remove_txt_messages_for_showing_file);

            next_reader_state.set(TxtReaderState::PreDisplaying);
            let title = raw_text.name.clone();
            let mut window = window.single_mut();

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
                    .with_children(|parent| create_txt_viewer(parent, font.clone(), title.clone()));
            });
            window.title = title;
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
