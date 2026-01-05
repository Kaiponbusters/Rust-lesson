pub fn run() {
    let res1 = division_option(5.0, 0.0);
    match res1 {
        Some(x) => println!("Result : {:.3}", x),
        None => println!("Not Allowed!!"),
    }

    let res2 = division_result(5.0, 2.0);
    match res2 {
        Ok(x) => println!("Result {}", x),
        Err(e) => println!("{}", e),
    }

    let a = [1, 2];
    let res3 = sum(&a);
    match res3 {
        Some(x) => println!("{}", x),
        None => println!("Out of Index!!"),
    }
}

// Optionでのエラーハンドリング
fn division_option(x: f64, y: f64) -> Option<f64> {
    if y == 0.0 { None } else { Some(x / y) }
}

// Result型でのエラーハンドリング
fn division_result(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        Err(String::from("Not Allowed!"))
    } else {
        Ok(x / y)
    }
}

// 存在しないインデックスを参照したときの回避
fn sum(a: &[i32]) -> Option<i32> {
    // ?マークはOutOfIndexのエラーをキャッチする
    let a1 = a.get(0)?;
    let a2 = a.get(1)?;
    let a3 = a.get(2)?;
    Some(a1 + a2 + a3)
}
