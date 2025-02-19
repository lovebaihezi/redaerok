use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use flume::{Receiver, Sender};

const TEXT_FOR_TESTING_APP: &str = "# 什么是后现代 (Postmodernism) ?
衡水中学就是现代性：锚定一个明确的世俗目标（考上好大学），在工具理性的指引下动用一切手段克服困难，争取胜利。在衡水模式里，学生日常的每一个细节都被关在理性的铁笼里，每天几点睡觉、几点起床、几点吃饭、几点跑操都被学校专家精确计算安排，唯一的原则就是以最高效的方式追求实现世俗目标。理论上人可以在里面过得很舒服，衣食无忧。除了这种生活看起来有点像养鸡场，非常无聊，时间久了大概会得精神衰弱。赛里斯是最后一个现代化国家，所以衡水中学是小号的赛里斯，赛里斯是大号的衡水中学。齐格蒙特·鲍曼要是来赛里斯，他无论如何也得写一本《现代性与衡水中学》。
赛里斯和西方国家的传统教育就是前现代：没有明确的学科划分，经史子集或《圣经》、古典文学中包含了几乎一切必须学的知识。这些知识大多关乎伦理道德、社会治理与宗教礼仪，在前现代社会里这些内容也不分家：政治经济和社会治理依靠道德和宗教，宗教和道德则在社会治理的过程中得以发挥作用。社会的主导原则是非世俗非物质性的宗教伦理或者超验道德，遵守传统的社会规范被认为是不需要质疑的常识。而这种传统致力于维护社会有机体的和谐稳定，而不是鼓励“创造性破坏”，哪怕代价是生活水平的稳定停滞。
现在四线县城中学里的杀马特青年们就是后现代：和前现代的类似之处在于不看重工具理性，他们从来不会思考“考不上985、找不到好工作怎么办”。区别则在于他们也不关心严格的道德秩序，一切硬性规定都是学校领导们对自己天性的压迫，是装B（学名规训），是权力的建构，所以主打一个随心所欲。但由于他们又没有什么真正美好的本性能被释放出来，所以只能尝试各种有违风化的事情，并且希望让全社会都相信“美应该由每一个人来定义”。就像半个世纪前一群法国后现代知识分子写联名信，拽了一大堆黑话就为了争取合法炼铜的自由。很多杀马特青年也不是没吃过衡水模式的苦，但最后选择了放弃。就像后现代都是被高度发达的现代文明滋养出来的一样。真到了世事维艰的时候，无所事事的该溜子往往是第一个活不下去的——就像蟑螂不能离开人类聚居区生存一样。";

#[derive(Debug, Clone, PartialEq, Eq, Hash, Resource)]
pub struct RawTxt {
    name: Option<String>,
    raw: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Component)]
struct Article {
    title: String,
    paragraphs: Vec<Paragraph>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Component)]
struct Paragraph {
    content: String,
}

impl Paragraph {
    fn new(content: impl ToString) -> Self {
        Paragraph {
            content: content.to_string(),
        }
    }
}

#[derive(Resource)]
pub struct Channel(Sender<Article>, Receiver<Article>);

impl From<&RawTxt> for Article {
    /// # Parse Simple Article from RawTxt
    /// Heavy Computing
    fn from(raw: &RawTxt) -> Self {
        let title = raw.name.clone().unwrap_or_default();
        let paragraphs = raw.raw.split("\n").map(Paragraph::new).collect();
        Article { title, paragraphs }
    }
}

pub fn setup_txt_viewer(mut command: Commands) {
    command.insert_resource(RawTxt {
        name: Some("后现代".to_string()),
        raw: TEXT_FOR_TESTING_APP.to_string(),
    });
    let (sender, receiver) = flume::unbounded::<Article>();
    command.insert_resource(Channel(sender, receiver));
}

pub fn handle_paragraphs(mut channel: ResMut<Channel>, raw_text: Res<RawTxt>) {
    let task_pool = AsyncComputeTaskPool::get();
    let channel = channel.as_mut();
    let sender = channel.0.clone();
    let text = raw_text.as_ref().clone();
    task_pool
        .spawn(async move {
            let article = Article::from(&text);
            sender.send_async(article).await.unwrap();
        })
        .detach();
}

pub fn txt_viewer_render_txt(mut channel: ResMut<Channel>) {
    let channel = channel.as_mut();
    let rec = channel.1.clone();
}

pub fn txt_viewer_scroll_viewer() {}
