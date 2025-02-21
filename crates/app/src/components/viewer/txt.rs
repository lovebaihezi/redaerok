use bevy::{prelude::*, tasks::AsyncComputeTaskPool, utils::futures};

use flume::{Receiver, Sender};

const TEXT_FOR_TESTING_APP: &str = "# 什么是后现代 (Postmodernism) ?
衡水中学就是现代性：锚定一个明确的世俗目标（考上好大学），在工具理性的指引下动用一切手段克服困难，争取胜利。在衡水模式里，学生日常的每一个细节都被关在理性的铁笼里，每天几点睡觉、几点起床、几点吃饭、几点跑操都被学校专家精确计算安排，唯一的原则就是以最高效的方式追求实现世俗目标。理论上人可以在里面过得很舒服，衣食无忧。除了这种生活看起来有点像养鸡场，非常无聊，时间久了大概会得精神衰弱。赛里斯是最后一个现代化国家，所以衡水中学是小号的赛里斯，赛里斯是大号的衡水中学。齐格蒙特·鲍曼要是来赛里斯，他无论如何也得写一本《现代性与衡水中学》。
赛里斯和西方国家的传统教育就是前现代：没有明确的学科划分，经史子集或《圣经》、古典文学中包含了几乎一切必须学的知识。这些知识大多关乎伦理道德、社会治理与宗教礼仪，在前现代社会里这些内容也不分家：政治经济和社会治理依靠道德和宗教，宗教和道德则在社会治理的过程中得以发挥作用。社会的主导原则是非世俗非物质性的宗教伦理或者超验道德，遵守传统的社会规范被认为是不需要质疑的常识。而这种传统致力于维护社会有机体的和谐稳定，而不是鼓励“创造性破坏”，哪怕代价是生活水平的稳定停滞。
现在四线县城中学里的杀马特青年们就是后现代：和前现代的类似之处在于不看重工具理性，他们从来不会思考“考不上985、找不到好工作怎么办”。区别则在于他们也不关心严格的道德秩序，一切硬性规定都是学校领导们对自己天性的压迫，是装B（学名规训），是权力的建构，所以主打一个随心所欲。但由于他们又没有什么真正美好的本性能被释放出来，所以只能尝试各种有违风化的事情，并且希望让全社会都相信“美应该由每一个人来定义”。就像半个世纪前一群法国后现代知识分子写联名信，拽了一大堆黑话就为了争取合法炼铜的自由。很多杀马特青年也不是没吃过衡水模式的苦，但最后选择了放弃。就像后现代都是被高度发达的现代文明滋养出来的一样。真到了世事维艰的时候，无所事事的该溜子往往是第一个活不下去的——就像蟑螂不能离开人类聚居区生存一样。";

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

#[derive(Resource)]
pub struct Channel(Sender<Paragraph>, Receiver<Paragraph>);

pub fn init_text_viewer(mut command: Commands, assests: Res<AssetServer>) {
    let font = assests.load("fonts/SourceHanSerifCN-VF.ttf");
    command
        .spawn((
            TxtBase,
            Node {
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
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
                        padding: UiRect::all(Val::Px(4.0)),
                        border: UiRect::bottom(Val::Px(0.5)),
                        overflow: Overflow::scroll_x(),
                        ..Default::default()
                    },
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
                    ..Default::default()
                },
            ));
        });
}

pub fn handle_new_text(mut command: Commands) {
    // TODO(chaibowen): Currently schedule in StartUp for testing, it should be triggered as recv
    // an event of a file selected and readed
    let raw_text = RawTxt {
        name: "后现代".to_string(),
        raw: TEXT_FOR_TESTING_APP.to_string(),
    };
    command.insert_resource(raw_text.clone());
    let (sender, receiver) = flume::unbounded::<Paragraph>();
    command.insert_resource(Channel(sender.clone(), receiver));
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

pub fn update_title_based_on_current_article(
    raw_text: Res<RawTxt>,
    txt_title_query: Query<&Children, With<TxtTitle>>,
    mut text_query: Query<&mut Text>,
) {
    for txt_title in &mut txt_title_query.iter() {
        let mut content = text_query.get_mut(txt_title[0]).unwrap();
        **content = raw_text.name.to_string();
    }
}

pub fn txt_viewer_render_txt(
    mut channel: ResMut<Channel>,
    mut command: Commands,
    raw_text: Res<RawTxt>,
) {
    let channel = channel.as_mut();
    let rec = channel.1.clone();
    let mut paragraph_async = rec.recv_async();
    match futures::check_ready(&mut paragraph_async) {
        Some(Ok(pragraph)) => {
            let content_indexes = pragraph.content;
            let raw_slice = &raw_text.raw[content_indexes[0]..content_indexes[1]];
        }
        Some(Err(_)) => {}
        None => {}
    }
}

pub fn txt_viewer_scroll_viewer() {}
