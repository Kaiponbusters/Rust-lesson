// traitは複数の方のタイプに対して共通する機能などを持たせたいときに利用
trait Fruits {
    fn price(&self) -> u32;
}
struct Apple;
impl Fruits for Apple {
    fn price(&self) -> u32 {
        10
    }
}
struct Banana;
impl Fruits for Banana {
    fn price(&self) -> u32 {
        5
    }
}

// Java/Kotlinのinterfaceみたいな感じ
trait Summary {
    fn summarize(&self) -> String {
        String::from("read more...")
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    // fn summarize(&self) -> String {
    //     //formatマクロは埋め込んだものをString型として返す
    //     format!("{}, by {} {}", self.headline, self.author, self.location)
    // }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} {}", self.username, self.content)
    }
}

trait Message {
    fn message(&self) -> String {
        String::from("Message")
    }
}

impl Message for NewsArticle {}

pub fn run() {
    let apple = Apple {};
    let banana = Banana {};
    get_price(apple);
    get_price(banana);
    let tweet = Tweet {
        username: String::from("kaipoooo"),
        content: String::from("My name is banana"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet : {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Banana is exploded"),
        location: String::from("Japan"),
        author: String::from("BananaMan"),
        content: String::from("Dekke Banana is Founded"),
    };
    println!("{}", article.summarize());
    notify(&article);
    notify_another(&article);
}

// Traitを持つデータ型のみを受け入れる関数
fn get_price<T: Fruits>(fruits: T) {
    println!("price is {}", fruits.price());
}

// 引数のデータ型に &implを設定している。
// Summary Traitを設定しているのであれば使える
// そのデータを引数で渡せる
fn notify(item: &impl Summary) {
    println!("Breaking News! {}", item.summarize());
}

// Summary + Messageの両方のTraitを持っている型のみ受け入れる
fn notify_another(item: &(impl Summary + Message)) {
    println!("Breaking News! {}", item.summarize());
    println!("Message {}", item.message());
}
