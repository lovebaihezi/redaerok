use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    picking::focus::HoverMap,
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
    utils::futures,
};

use flume::Receiver;

const TEXT_FOR_TESTING_APP: &str = r#"
1 
　　第一次听说鲁迅这名字是一谜语：山东消息--打一人名，忘了发表在哪儿，反正是一印刷纸，一大堆谜语，让小孩猜。大约八九岁的时候，我们院一爱看书的孩子跟我们一帮人吹：有一鲁迅，太牛逼了。他眉飞色舞地说：丫行于一条黑巷，一群狗冲丫叫，丫说：呸！你这势利的狗。我和一干听众大笑，当时我刚被304医院一只三条腿的狗追过，吓得不轻，这句话对我的心理大有抚慰。有那么几周，我们上下学，谁走在后面，前面的人就会回头笑骂：呸！你这势利的狗。 

　　第一本鲁迅的书就是这孩子借给我看的，不是《野草》便是《热风》或是另一本，上面有骂狗这一段。我一向有一特异功能，无论什么书，拿来一翻，必先翻出涉嫌黄色那一段。鲁迅的书也不例外，一翻翻到一篇杂文，主要内容是摘抄当年一份流氓小报登载的社会新闻，说的是上海一妇人诉上法庭告其夫鸡奸，似乎引的是原始卷宗。我当时是一特别正经的人，就是那种对这类下流故事爱看，看完之后又奋起谴责的家伙。我对鲁迅文风的第一观感并不十分之好，如此文摘怎么能算他的东西？有一种人写文章专爱引用别人的话，当时是一时弊，现在还是一俗例，起先我把鲁迅也当成了这种人。 

　　后来开始看鲁迅的小说，文化大革命焚书坑儒，可是没烧鲁迅的书，书店里除了毛泽东选集马恩列斯全集剩下的就是鲁迅全集赫然摆在那里。老实讲，当时很容易崇拜个谁，《艳阳天》我都觉得好，但是并没觉得鲁迅的小说写得好，可能是因为那时我只能欣赏戏剧性强和更带传奇性的作品，对人生疾苦一无所知，抱着这样自我娱乐的心态看书，鲁迅的小说就显得过于沉闷。相对于北京孩子活泼的口语，鲁迅那种二三十年代正处于发轫期尚未完全脱离文言文影响的白话文字也有些疙疙瘩瘩，读起来总有些含混，有些字现在也不那么用了，譬如把"的"一律写做"底"，好像错别字似的，语气也变得夹生。这就是大师啊？记得我当时还挺纳闷。再后来，阅读的经验增加了，自己也写了二十年小说，对小说也不简单地用明白流畅情节生动当唯一标准了，我要说，鲁迅的小说写得确实不错，但不是都好，没有一个作家的全部作品都好，那是扯淡。而且，说鲁迅的小说代表中国小说的最高水平，那也不是事实。 

　　我觉得鲁迅写得最另类的三篇小说是《一件小事》、《狂人日记》和《伤逝》。《一件小事》从立意到行文都很容易被小学生模仿，这篇东西也确实作为范文收入过小学课本，像小说结尾那句"他的背影高大起来"，我那个不学无术的女儿在她的作文中就写过。写《狂人日记》时鲁迅充满文学青年似的热情，文字尚嫌欧化，透着刚睁开眼睛看世界的吃惊，那种激烈决绝的态度则和今天的"愤青"有共通之处，搁今天，也许能改编成摇滚。《伤逝》大概是最不像鲁迅后来风格的一部小说，男女过日子的事儿，他老人家实在是生疏，由此可见，大师也有笔到不了的地方，认识多么犀利也别想包打天下。 

　　《从百草园到三味书屋》和《社戏》是很好的散文，有每个人回忆童年往事的那份亲切和感伤，比《荷塘月色》、《白杨礼赞》什么的强很多，比史铁生的《我与地坛》可就不是一个量级了。那也不在作家的经验、才华，在于不同人生本身的差距。 

　　《祝福》、《孔乙己》、《在酒楼上》和吃血馒头的那个《药》是鲁迅小说中最好的，和他同时代的郁达夫、沈从文和四川那位写《死水微澜》的李劼 人有一拚，在当时就算是力透纸背的。中国普通人民的真实形象和难堪的命运被毫不留情地端了出来。这些人物至今刺激着我们，使我们一想到他们就毫无乐观的理由。半个世纪之后，我们的人民不再是鲁迅那个时代完全处于被忽略被遗忘的境地很需要被同情的那伙人了。从鲁迅第一声呐喊起，他们也折腾了几十年，再提到人民二字，只怕要警惕一点了，有些事是别人强加的，有些事可是他们自个乐意的，甚至还有不少诗意的发挥。仅有唤醒意识和对压迫者的控诉那都是表面文章，真正需要勇气和胆识的不是反抗强者，而是直面那些可怜的、被侮辱被损害的人，对他们予以解剖。 

　　鲁迅写小说有时是非常概念的，这在他那部备受推崇的《阿Q正传》中尤为明显。小时候我也觉得那是好文章，写绝了，活画出中国人的揍性，视其为揭露中国人国民性的扛鼎之作，凭这一篇就把所有忧国忧民的中国作家甩得远远的，就配去得诺贝尔奖。这个印象在很长时间内抵消了我对他其他作品的怀疑，直到有一次看严顺开演的同名电影，给我腻着了。严顺开按说是好演员，演别的都好，偏这阿Q怎么这么讨厌，主要是假，没走人物，走的是观念，总觉得是在宣传什么否定什么昭示什么。在严顺开身上我没有看到阿Q这个人，而是看到了高高踞于云端的编导们。回去重读原作，发现鲁迅是当杂文写的这个小说，意在针砭时弊，讥讽他那时代一帮装孙子的主儿，什么"精神胜利法"、"不许革命"、"假洋鬼子"，这都是现成的概念，中国社会司空见惯的丑陋现象，谁也看得到，很直接就化在阿Q身上了，形成了这么一个典型人物，跟马三立那个"马大哈"的相声起点差不多。当然，他这信手一拈也是大师风范，为一般俗辈所不及，可说是时代的巨眼那一刻长在他脸上，但我还是得说，这个阿Q是概念的产物，不用和别人比，和他自己的祥林嫂比就立见高下。概念形成的人物当作认识的武器，针对社会陋习自有他便发发扬火力指哪儿打哪儿的好处，但作为文学作品中的审美对象他能激起读者的情感反应就极为有限了。是不是有这么一个规律，干预性针对性越强的作品，审美性可感性就越低？尤其是改编为影视这种直接出形象艺术形式，这类人物就很吃亏，演员也很难从生活中找依据。 

　　鲁迅有一批小说游戏成分很大，我指的是他那本《故事新编》。这是我最喜欢的一批作品。这些游戏之作充分显示了鲁迅的才气和机灵劲儿，再加上一条就是他深厚的旧学知识。这也不是随便什么人能写的，他对历史和历史人物的态度真够姚雪垠凌解放包括陈家林学半年的。若说鲁迅依旧令我尊敬，就是他对什么样公认的伟大人物也没露出丝毫的"奴颜和媚骨"，更没有用死无对证的方法大肆弘扬民族正气，编织盛世神话。他对历史故事和历史人物的怀疑渗透在《故事新编》的第一笔中。唯一叫人败兴的是编者在这批小说下面加的注释，告诉今人这话指什么，那段是讽刺当时的什么现象，那就变得小气了，纯粹是意气用事，借古讽今。有些话我本不想说，但话赶到这儿了，我还是说了吧。鲁迅这个人，在太多人和事上看不开，自他去了上海，心无宁日，天天气得半死，写文章也常跟小人过不去。愤怒出诗人，你愤怒的对象是多大格局，你的作品也就呈现出多大格局。鲁迅的个性在他的创作中刻下了深深的烙印，此外，他的文学理念也不可避免地受到时候潮流的影响和摆布。 

2 
　　在某些方面，我的观念很保守，譬如作家这个称呼，我一直认为必须写小说才配这么自称。（诗人单算，他们可以直接叫诗人。）我是把小说当作"作家"这一行的防伪标记看待的，因为有太多不着调的人在写散文。凡见报的中国作家代表团名单中顶着"著名散文作家"头衔那位往往是一冒牌货，不是作协官员就是某人的儿子或者干脆是文学圈里一碎催，能写个山水游记或是某老腕某年某日一时的音容笑貌就覥 着脸出来招摇了。这些人严重败坏了散文随笔的名声，使我一想到散文随笔总觉得那不是正经东西，若说某人独以散文随笔见长，先觉得这是一起哄的，读了也以为好，仍觉得此人没根基，起码和文学无关，用那种比较装孙子的话说就是"文化意义大于文学意义"。 

　　关于这一点，我和一个作家朋友当面争论过，我认为鲁迅光靠一堆杂文几个短篇是立不住的，没听说有世界文豪只写过这点东西的。我这朋友说：我坚决不同意你这说法！接着举到另一位也是很多人的精神之父阿根廷人博尔赫斯为例，这位也是没写过多少东西便一举成事儿的。这倒弄得我没话可说。当然我并没有被说服，我也没觉得博尔赫斯怎么着了。我坚持认为，一个正经作家，光写短篇总是可疑，说起来不心虚还要有戳得住的长篇小说，这是练真本事，凭小聪明雕虫小技蒙不过去。有一种为没写过什么东西混了一辈子的老作家遮丑的鬼话，说写短篇比写长篇难，因为结构如何如何之难，语言如何如何精练，这也就是蒙蒙没写过东西的人。短就是短，长就是长，写长的要比写短的多倾注心血这还用说么？长篇就不用结构了？就该罗嗦？长篇需要用力劳神的地方那是只会写短篇的人想也想不到的。是，小说只有好坏之分，不在长短，同是好小说，我也没见过谁真拿《祝福》、《交叉小径的花园》去和《红楼梦》、《追忆逝水年华》相提并论。 

　　鲁迅没有长篇，怎么说都是个遗憾，也许不是他个人的损失，而是中华民族的损失。以他显露的才能，可以想象，若他真写长篇，会达到一个怎样的高度。这中间有一个悖论：如果不是那样一个乱世，周围有那么多叫他生气的人和事，他再不是那么个脾气，他也就有时间写长篇了；但若不是那样一个时代，周围不是那么个环境，他也跟他弟一样客气，我们就只有在翻阅北洋政府人事档案时才能找到周树人的名字，知道是那个周作人的哥。所以这也是中国文学的宿命，在鲁迅身上，我又看到了一个经常出现的文学现象，我们有了一个伟大的作家，却看不到他更多优秀的作品。 

3 
　　在我小时候，鲁迅这个名字是神圣的，受到政治保护的，"攻击鲁迅"是严重的犯罪，要遭当场拿下。直到今天，我写这篇东西，仍有捅娄子和冒天下之大不韪的感觉。人们加在他头上无数美誉：文豪！思想先驱！新文化运动主将！骨头最硬！我有一个朋友一直暗暗叫他"齐天大圣"。我们都知道，他对中国的贡献并不局限于文学，他是有思想的。思想和作家不是一个等号关系，作家，能写文学作品，不见得有思想，要想当最顶级的作家，必须有思想，这个我们从小就分得清，也就是说，思想是首要的，大于其他的。当然还有先进思想和落后思想之分，这且不管他，鲁迅，自然是最先进的，这个认识被当作铁的事实早就灌输到我的头脑之中。 

　　像所有被推到高处的神话人物一样，在鲁迅周围始终有一种迷信的气氛和蛮横的力量，压迫着我们不能正视他。他是作为一个不可言说的奇迹存在的。在我读过他的大部分作品并已得出自己的看法之后的很长时间，仍不能摆脱对他的迷信，一想到他就觉得他的伟大是不证自明的。如果说他的作品不是很过硬，那他还有过硬的思想，那个思想到今天还闪烁着锋利的光芒，照耀着我们黑暗的自身。我以为我了解他的思想，实际上我没有读过任何他的思想著作，一些专用于他的句子使我觉得不必深究，"一个都不宽恕！""横眉冷对千夫指，俯首甘为孺子牛。""殖民地半殖民地人民最可贵的品格，没有丝毫的奴颜和媚骨。"--这不就是思想么？ 

　　思想解放运动开始后，老百姓第一个变化就是嘴坏了，谁都敢说。深圳建特区后，我有一个做律师的朋友去那边捞世界，回来之后请大家吃饭，有人喝了酒后高叫：鲁迅，有什么呀！论思想，他有毛泽东有思想吗？毛泽东，有雄文四卷，起码让三代中国人灵魂受到洗礼；论骨头硬，他有王二小骨头硬吗？给敌人带路，掩护了几千老乡和干部，被敌人摔死在石头上。 

　　我不是说这酒鬼说的话多么发人深省，真正使我震动的是他的态度，不一定非要正确才能发言，怎么想的就怎么说，说了也就说了，破除迷信解放思想确实先要有这么个耍王八蛋的过程。 

　　这使我终于可以用一个人看另一个人的眼光去打量鲁迅。这时我才发现我对他有多不了解。那些经常用于称赞他的话其实不属于思想，只是夸他的为人或说高贵的德行，拜倒在他的光芒之下那么久其实我对他的思想一无所知。从他无数崇拜者的文章中我也想不想谁说过他有思想，大家纠缠、感慨、为之涕下、激动不已的大都是他的品格，最厚道的文章也只是对他可能具有的思想进行猜测，想象这样一个为世不容、痛苦敏感的智者内心一定是"漆黑一团"，这个逻辑似乎是说，对生活、社会、人群极度绝望本身就是深刻的思想。我不是太明白这个逻辑，坦白说，直到昨天，写到这里，我还是晕菜，不知道鲁迅思想的精髓到底是什么。 

　　我一有位常在一起吃吃喝喝的朋友一直对鲁迅怀有一些私人兴趣，收集有很全的鲁迅资料，很多关于鲁迅的闲话我都是听他讲的，于是我专门向他请教，鲁迅有什么思想？这位朋友似乎也懵了一下，想了想说，实际也没什么新鲜的，早期主张"全盘西化"，取缔中医中药，青年人不必读中国书；晚年被苏联蒙了，以为那儿是王道乐土，向往了好一阵，后来跟"四条汉子"一接触，也发觉不是事儿。据鲁迅最新研究成员讲，鲁迅是主张"人权"的，是"自由主义知识分子"，因为毕竟写过《论"费尔泼赖"应该缓行》，鲁研家们还没找出办法将他归到英国式消极自由那一筐里。如此等等，胡说一气，当时我是满足了，回到家里坐在电脑前还是糊涂，对"思想"这个词的包含范围感到糊涂，不能说给国家民族指条明道不叫思想，但我对鲁迅的期待和他一直享有的地位似乎又不应仅限于此。在此，我觉得自己挺可悲的，那么痴心地笃信过很多不甚了了的东西，其实不明真相，还是那里磕头如捣蒜，就怕别人说自己浅薄。 

4 
　　说到鲁迅精神，这个我是知道的，就是以笔为旗，以笔为投枪或匕首，吃的是草，挤的是奶，痛打落水狗，毫不妥协地向一切黑暗势力挑战。与之相联的形象便是孤愤、激昂、单枪匹马，永远翻着白眼，前面是一眼望不到头的明枪，身后是飞蝗一般放不完的冷箭，简言之，战士的一生。有一句话，本是他贴赠孙中山的，后多为他那些爱好者回赠于他：有缺点的战士依然是战士，完美的苍蝇不过是苍蝇。林语堂也形容过鲁迅：不交锋则不乐，不披甲则不乐，即使无锋可交，无矛可持，拾一石子投狗，偶中，亦快然于胸中。此鲁迅之一副活形也。 

　　这个不会为缺点玷污逮谁跟谁急的战士形象对后代中国作家的吸引远大于写小说的那个鲁迅。大家似乎达成了一个共识，只会写小说的作家是低级动物，做战士才是清名永留的不二法门，甚至是把一举成名的金钥匙。于是，忿于世人不肯受他超度的传道士来了，才尽落魄的三流文人来了，大事小事一直不顺的倒霉蛋、心理变态的自大狂和一班普普通通的愤世嫉俗者都来了。什么样的病人一集合，毛病都不叫毛病，改叫众人皆醉我独醒了。 

　　我觉得这个风气特别不好，理应拓荒自耕富而不骄的文坛成了小商小贩云集叫卖的市场。很多有才只是一时手背的作家彻底可惜了。北京有个毛老师，原来的小说写得不错，号称天下速度第一，五千言字一杯茶工夫立等可取，我是见这个名字就买，每读必有心得。近两年入了此道，天南海北危言耸听，看上去已与猛张飞无异，所言之事，对不起，尽是别人喝剩的茶根儿，大医院倒出的药渣儿。还有那一伙子在校不在校的家伙，竞相出一些大话集，名为书生实为书商，一写小说便露了馅儿，博士学位也要印在书皮儿上，明明是讨饭的花招偏要自称"挑战"，不知道那叫寒碜吗？在这我确实要以前辈的口气对他们说几句：有志气，允许；想当作家，可以；走正道。读书尽可以使人无耻，但自己要给自己设一个底线，丢人的事也有瘾，干过一次就想着下次。 

　　还有那个伊沙，出了本书，直接就叫《一个都不宽恕》。鲁迅对伪君子假道学种种愚昧麻木中国人的劣根性骂得都对，若说还有遗珠之憾，就是把自己拉下了。伊沙那儿就拉了两个人，一个鲁迅，一个他自己。这就不彻底了，一本书的风格也很不统一，一半骂别人，一半夸自己，诗也上了，脑子盘算过的文学构想也拿出来了，历数自己的种种仗义，这就没劲了。 

　　我觉得这是一个很重要的问题，涉及到人之为人的根本立场。说众人皆醉我独醒可以，说众人皆浊我独清，这个恐怕只有刚出生的婴儿才配。依我之见，中国人最大的劣根性就是乌鸦落在猪身上--光看见别人黑了。物理学早就证明了，在这个地球上没有一个人处于比其他人优越的地位，代替上帝对别人进行精神审判，在笃信宗教的国家是最大的渎神。缺点就是缺点，譬如病菌，无论是战士还是苍蝇携带都会使人生病。 

　　后人的效颦都要鲁迅负责并不公平。这就是榜样的悲哀，遭人热爱看来也不全是美事。鲁迅对自己到底怎么看，大概我们永远不知道了。有一点也许可以肯定，倘若鲁迅此刻从地下坐起来，第一个耳光自然要扇到那些吃鲁迅饭的人脸上，第二个耳光就要扇给那些"活鲁迅"、"二鲁迅"们。 

5 
　　阿Q讲过：尼姑的光头，和尚摸得，我就摸不得么？对鲁迅，我也这么想。各界人士对他的颂扬，有时到了妨碍我们自由呼吸的地步。我不相信他如此完美，没有这样的人，既然大家载来越严厉地互相对待，他也不该例外。他甚至应该成为一个标尺，什么时候能随便批评他了，或者大家都把他淡忘了，我们就进步了。中国有太多的神话，像我这样的红尘中人，若想精神自由，首先要忘掉还有一个"精神自由之神"。 

　　我的那个研究鲁迅的朋友对我说：鲁迅是相信进化论的，即未来比现在好，青年人比老年人好。他还讲，他的使命就是扛住正往下落的闸门，让年轻人能逃出一个算一个。后来在广州厦门看到清党，他这个观念有些动摇，认为青年人坏起来也不逊于老的。但到临死，他还是对未来抱有信心，一次看到苏联红场阅兵的纪录片，对许广平和在场的萧红说：这个场面我是看不到了，也许你们能看到，海婴能看到。 

　　这位朋友再三对我说：他其实是很热情的，很热情的。 

| 2000年1月25日定稿

"#;

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

pub fn init_text_viewer(mut command: Commands, assests: Res<AssetServer>) {
    let font = assests.load("fonts/SourceHanSerifCN-VF.ttf");
    command
        .spawn((
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

#[derive(Component)]
struct RawTxtAsync(Task<Option<RawTxt>>);

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

pub fn handle_new_text(mut command: Commands) {
    // TODO(chaibowen): Currently schedule in StartUp for testing, it should be triggered as recv
    // an event of a file selected and readed
    let raw_text = RawTxt {
        name: "我看鲁迅".to_string(),
        raw: TEXT_FOR_TESTING_APP.to_string(),
    };
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
