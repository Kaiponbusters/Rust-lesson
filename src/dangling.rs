pub fn run() {
    println!("\n===== Dangling Pointer / Reference (Rust) =====");

    safe_rust_prevents_dangling();
    unsafe_can_create_dangling_raw_pointer();
    safe_alternatives();
}

fn safe_rust_prevents_dangling() {
    println!("\n-- 1) safe Rust ではダングリング参照は基本的に作れない --");

    println!(
        "例：関数内で作った値への参照を返す（これはコンパイルエラーになる）\n{}",
        r#"fn dangling() -> &String {
    let s = String::from(\"hello\");
    &s // s は関数終了時に drop されるため、返す参照がダングリングになる
}"#
    );

    println!(
        "\nRustは『参照は参照先より長生きできない』をライフタイムで検査するので、\nこの手のコードはビルド時に止められる。"
    );

    // ここから下は実際に動く safe な例（ダングリングにならない）。
    let s = String::from("hello");
    let r = &s;
    println!("safe borrow: r = {}", r);
}

fn unsafe_can_create_dangling_raw_pointer() {
    println!("\n-- 2) unsafe（生ポインタ）だとダングリング“状態”を作れてしまう --");

    let s = String::from("hello");
    let p = s.as_ptr();
    let len = s.len();

    println!("before drop: heap ptr = {:p}, len = {}", p, len);

    // s を明示的に解放する（スコープ終端でも同様）。
    drop(s);

    // ここで p は『解放済みのヒープ領域』を指している可能性があり、ダングリング。
    println!(
        "after drop:  heap ptr = {:p} (このポインタはダングリングの可能性がある)",
        p
    );

    // 注意: ここで p を逆参照して読むのは未定義動作(UB)になり得る。
    // 実行してしまうと環境によってはクラッシュ/データ破壊/たまたま動く、などが起こる。
    //
    // unsafe {
    //     let first_byte = *p; // <-- UBになり得るのでやらない
    //     println!("{}", first_byte);
    // }

    println!(
        "ポイント：unsafe は『コンパイラが安全性を保証しない』だけで、\n安全になるわけではない。正しさは書いた側の責任になる。"
    );
}

fn safe_alternatives() {
    println!("\n-- 3) 実務でよく使う安全な代替 --");

    println!("(A) 参照を返さず、所有権（値）を返す");
    fn return_owned() -> String {
        let s = String::from("hello");
        s
    }
    let owned = return_owned();
    println!("owned = {}", owned);

    println!("\n(B) 引数の参照から『同じ入力の一部』を返す（入力より長生きしない）");
    fn first_word(s: &str) -> &str {
        match s.split_whitespace().next() {
            Some(w) => w,
            None => "",
        }
    }

    let text = String::from("hello world");
    let w = first_word(&text);
    println!("first_word = {}", w);
}
