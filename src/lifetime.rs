pub fn run() {
    let st1 = String::from("x");
    let st2 = String::from("y");
    let res1 = get_longest(&st1, &st2);
    println!("{}", res1);

    let st3 = String::from("x");
    let res2;
    {
        let st4 = String::from("y");
        res2 = get_longest(&st3, &st4);
        println!("{}", res2);
    }
    // ここにprintが入ると、res2は生きていないst4を参照しようとしてダングリングエラーが出る
}

// 'aという「GenericなLifetimeパラメータ」を設定
// Rustの参照型は実は &'a T / &'a mut T みたいな形
// 'a は「その参照が有効である期間」を示す型パラメータ
// 'a はx,yの両方が生きている範囲を超えられない
// つまり、'a は戻り値が引数と同じ期間だけ有効という関係を型として表すために使う。
fn get_longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// 関数で実態を作ってrefをreturnで出すと、sはdropしてしまう。
// refが独り歩きでダングリングエラーが発生。
// fn dummy1<'a>() -> &'a str {
//     let s = String::from("demo");
//     &s
// }
// fn dummy2<'a>() -> &'a i32 {
//     let x = 10;
//     &x
// }

//これはそもそも独り歩きしない
// fn dummy3() -> String {
//     let s = String::from("demo");
//     s
// }