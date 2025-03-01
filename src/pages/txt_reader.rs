use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
    utils::futures,
};
use rfd::FileHandle;

use crate::{
    components::{
        button::normal_button::NormalButton,
        viewer::txt::{create_txt_viewer, Paragraph, ParagraphRecv, RawTxt, TxtBase},
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
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
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

pub fn manage_text_ui(
    mut commands: Commands,
    page_state: Res<PageState>,
    txt_ui: Query<Option<Entity>, With<TxtReader>>,
    mut title: Query<Option<&mut Text>, With<ReadUITitle>>,
) {
    info!("Page State: {:?}", *page_state);
    match *page_state {
        PageState::TxtReadPage(TxtReaderState::None) => {
            if txt_ui.is_empty() {
                commands.spawn(txt_ui_base()).with_children(|parent| {
                    parent.spawn(top_banner()).with_children(back_root);
                    parent
                        .spawn(txt_message_body())
                        .with_children(txt_messages_with_btn);
                });
            }
        }
        PageState::TxtReadPage(TxtReaderState::WaitForRFD) => {
            title.iter_mut().flatten().for_each(|mut title| {
                *title = "Opening RFD".into();
            });
        }
        PageState::TxtReadPage(TxtReaderState::WaitForUserSelecting) => {
            title.iter_mut().flatten().for_each(|mut title| {
                *title = "Wait For User Select file".into();
            });
        }
        PageState::TxtReadPage(TxtReaderState::WaitForLoadingFile(ref file_name)) => {
            if let Some(mut title) = title.single_mut() {
                *title = format!("Loading file: {}", &file_name).into();
            }
        }
        PageState::TxtReadPage(TxtReaderState::PreDisplaying) => {
            txt_ui.iter().flatten().for_each(|entity| {
                commands.entity(entity).despawn_recursive();
            });
        }
        _ => txt_ui.iter().flatten().for_each(|entity| {
            commands.entity(entity).despawn_recursive();
        }),
    }
}

pub fn on_click_back_to_root_btn(
    mut page_state: ResMut<PageState>,
    mut query: Query<(&Interaction, &BackToRootBtn)>,
) {
    for (interaction, _) in query.iter_mut() {
        if *interaction == Interaction::Pressed {
            *page_state = PageState::WelcomePage;
        }
    }
}

#[derive(Component)]
pub struct FileHandleAysnc(Task<Option<FileHandle>>);

#[derive(Component)]
pub struct RawTxtAsync(Task<RawTxt>);

pub fn on_click_open_local_file(
    mut query: Query<(&Interaction, &OpenFilePickerBtn)>,
    mut command: Commands,
    mut state: ResMut<PageState>,
) {
    for (interaction, _) in query.iter_mut() {
        if *interaction == Interaction::Pressed {
            *state = PageState::TxtReadPage(TxtReaderState::WaitForRFD);
            let pool = AsyncComputeTaskPool::get();
            let file_handle: Task<Option<FileHandle>> = pool.spawn(async move {
                let afd = rfd::AsyncFileDialog::new();
                afd.add_filter("text", &["txt", "md"]).pick_file().await
            });
            command.spawn(FileHandleAysnc(file_handle));
            *state = PageState::TxtReadPage(TxtReaderState::WaitForUserSelecting);
        }
    }
}

pub fn read_file(
    mut command: Commands,
    mut file_handles: Query<(Entity, &mut FileHandleAysnc)>,

    mut state: ResMut<PageState>,
) {
    for (ent, mut task) in file_handles.iter_mut() {
        if let Some(Some(handle)) = futures::check_ready(&mut task.0) {
            *state = PageState::TxtReadPage(TxtReaderState::WaitForLoadingFile(handle.file_name()));
            command.entity(ent).despawn_recursive();
            let pool = AsyncComputeTaskPool::get();
            let raw_txt: Task<RawTxt> = pool.spawn(async move {
                let raw_txt = handle.read().await;
                RawTxt {
                    name: handle.file_name().to_string(),
                    raw: String::from_utf8(raw_txt).unwrap(),
                }
            });
            command.spawn(RawTxtAsync(raw_txt));
        }
    }
}

pub fn handle_new_text(
    mut command: Commands,
    mut raw_txt_tasks: Query<(Entity, &mut RawTxtAsync)>,
    mut window: Query<&mut Window>,
    assets: ResMut<AssetServer>,
    body: Query<Entity, With<TxtUIBody>>,
    mut state: ResMut<PageState>,
) {
    let font = assets.load("fonts/SourceHanSerifCN-VF.ttf");
    for (ent, mut task) in raw_txt_tasks.iter_mut() {
        if let Some(raw_text) = futures::check_ready(&mut task.0) {
            *state = PageState::TxtReadPage(TxtReaderState::WaitForRFD);
            command.entity(ent).despawn_recursive();
            let title = raw_text.name.clone();
            let mut window = window.single_mut();
            window.title = title;

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

            *state = PageState::TxtReadPage(TxtReaderState::PreDisplaying);
            command.run_system_cached(manage_text_ui);
            let body_entity = body.single();
            command.entity(body_entity).with_children(|parent| {
                parent
                    .spawn(TxtBase::render(TxtBase))
                    .with_children(|parent| create_txt_viewer(parent, font.clone()));
            });
        }
    }
}
