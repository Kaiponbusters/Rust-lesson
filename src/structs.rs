#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn create(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    // 関数の引数をselfとした場合、対象インスタンスへのMethodとして呼び出せる。
    // &selfとするとインスタンスの参照を受け取る。
    // 付けない場合にはあるインスタンスから呼ばれたとき、そのインスタンスの所有権が引数に移動する
    fn area(&self) {
        println!("{}", self.width * self.height);
    }
}

pub fn run() {
    // 構造体をインスタンス化
    // インスタンスにも所有権が発生するよ。
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("aaaaaa@aaaaaaa,com");
    // // ここで所有権がuser1 -> user2移動してるよ
    // let user2 = user1;

    println!("{:#?}", user1);

    let user2 = build_user(String::from("user2@aaaa.aaa"), String::from("user2"));
    println!("{:#?}", user2);

    let rect = Rectangle::create(20, 20);
    println!("{:#?}", rect);
    rect.area();
    println!("{:#?}", rect);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
