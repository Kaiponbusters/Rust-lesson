pub fn run() {
    //s1が所有権を保持
    let s1 = String::from("hello");
    let s2 = s1;

    // このタイミングでs1の所有権はs2に移動済み
    // Copy Traitが実装されているのはStackで完結する型がほとんど
    println!("{}", s2);

    //以下はスタック上で管理しているため、Copyが起きているので動作する
    //整数型はCopy Traitを実装している
    let i1 = 1;
    let i2 = i1;
    println!("{} {}", i1, i2);
    println!("Stack address of i1 is : {:p}", &i1);
    println!("Stack address of i2 is : {:p}", &i2);

    let sl1 = "literal";
    let sl2 = sl1;

    println!("{} {}", sl1, sl2);
    println!("Stack address of sl1 : {:p}", &sl1);
    println!("Stack address of sl2 : {:p}", &sl2);

    // Cloneをすることで、同じものを別の領域にコピー可能
    let s3 = String::from("hello");
    let s4 = s3.clone();
    println!("Stack address of s3 : {:p}", &s3);
    println!("Stack address of s4 : {:p}", &s4);
    println!("heap address of s3 : {:p}", s3.as_ptr());
    println!("heap address of s4 : {:p}", s4.as_ptr());
    println!("{} {}", s3, s4);

    // 関数の引数にした場合でも所有権は移動する。
    // s5はtake_ownership()前はs5に所有権があるが、
    let s5 = String::from("hello");
    println!("Stack address of s5 : {:p}", &s5);
    println!("Heap address of s5 : {:p}", s5.as_ptr());
    println!("Len of s5 :{}", s5.len());
    println!("Cap of s5 :{}", s5.capacity());
    take_ownership(s5);
    // take_ownership()後にはsという引数に所有権が移動している
    // 下記のコードはエラーになる。
    // println!("{}", s5);

    let s6 = String::from("hello");
    println!("Stack address of s6 : {:p}", &s6);
    println!("Heap memory address of s6 : {:p}", s6.as_ptr());
    println!("Len of s6 : {}", s6.len());

    let s7 = take_giveback_ownership(s6);
    println!("Stack address of s7 : {:p}", &s7);
    println!("Heap memory address of s7 : {:p}", s7.as_ptr());
    println!("Len of s7 : {}", s7.len());

    // s8, lenともに利用することができる。
    // s8はImutableなRef
    let s8 = String::from("hello");
    let len = calculate_length(&s8);
    println!("The length of {} is {}", s8, len);

    // 参照はデータを読みに行くだけではなく、変更にも使える。
    // mutableなref（参照）
    let mut s9 = String::from("Hello");
    change(&mut s9);
    println!("{}", s9);

    // ImuなRefは複数作ることができる。
    let s10 = String::from("hello");
    let r1 = &s10;
    let r2 = &s10;
    println!("{} {} {}", s10, r1, r2);

    // ImuなRefとMutなRefは共存しない
    // データを参照しているものが一つでもあればデータを変更できなくする規制
    // let mut s10 = String::from("hello");
    // let r1 = &s10;
    // let r2 = &mut s10;
    // println!("{} {} {}", s10, r1, r2);

    // s11の所有権者であっても、Mutな所有権が有効な領域では所有権者は値を読めない。
    let mut s11 = String::from("hello");
    let r1 = &mut s11;
    println!("{}", r1);
    println!("{}", s11);

    // printした後にr1, r2を使用しなければimuが存在してもmutを作れる
    let mut s12 = String::from("hello");
    let r1 = &s12;
    let r2 = &s12;
    println!("{} and {}", r1, r2);
    // これ以降r1, r2を使わなければMutな参照を作れる。
    let r3 = &mut s12;
    // 参照外しで値の変更も可能。
    *r3 = String::from("hello_update");
    println!("{}", r3);
    println!("{}", s12);

    // sはスコープを抜けると解放される。
    fn take_ownership(s: String) {
        println!("Stack address of s : {:p}", &s);
        println!("Heap address of s : {:p}", s.as_ptr());
        println!("Len of s :{}", s.len());
        println!("Cap of s :{}", s.capacity());
        println!("{}", s);
    }

    fn take_giveback_ownership(s: String) -> String {
        s
    }

    // 関数の引数に所有権がわたるのは不便。
    // 引数を参照型にすれば解決する。
    fn calculate_length(s: &String) -> usize {
        s.len()
    }

    fn change(s: &mut String) {
        s.push_str("_world");
    }
}
