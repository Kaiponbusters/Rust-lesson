use std::vec;
struct Point<T> {
    x: T,
    y: T,
}
// 構造体で違うデータ型を取りたい場合
struct PointAnother<T, U> {
    x: T,
    y: U,
}
// Implementationを使ってMethodsを追加する事もできる
impl<T, U> PointAnother<T, U> {
    // 一つの構造体に対してインスタンスを複数生成することができる
    // selfはmixupMethodが実行されるとき、どのインスタンスから実行されているかを確認するため
    fn mixup<V, W>(self, other: PointAnother<V, W>) -> PointAnother<T, W> {
        // selfは元のインスタンスのxの値
        // otherは引数として渡されるyの値
        PointAnother {
            x: self.x,
            y: other.y,
        }
    }
}

pub fn run() {
    let number_list = vec![34, 50, 100, 62, 35];
    // let mut largest = number_list[0];

    // for number in number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }

    // println!("{}", largest_i32(number_list));

    // charでもlargestを使いたい...
    // genericsを使えば解決！
    let char_list = vec!['a', 'b', 'c', 'd'];
    println!("{}", largest(char_list));
    println!("{}", largest(number_list));

    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1.0, y: 2.0 };
    let p3 = PointAnother { x: 1, y: 2.0 };
    let p4 = PointAnother { x: "Rust", y: 'a' };
    let p5 = p3.mixup(p4);
    println!("{} {}", p5.x, p5.y);
}

// Vec型のlistから最大値を返す
fn largest_i32(list: Vec<i32>) -> i32 {
    let mut largest = list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// genericsを利用したlargest関数
// あらゆる型 <T> をつける
// Trait境界を設定することでGenericsで受け取るデータ型を絞れる（比較が使える型のみを受け取る）
fn largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
