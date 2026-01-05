const MAX_POINTS: u32 = 100_000;

pub fn run() {
    println!("vars called!");
    let mut x = 5;
    println!("the value x is {}", x);
    x = 6;
    println!("the value x is {}", x);

    let _i1 = 3;
    let _f1 = 0.1;

    println!("{}", usize::BITS);
    println!("memory address is const of {:p}", &MAX_POINTS);

    let i2: i64 = 1;
    let i3: i64 = 2;
    println!("Stack address of i2 is : {:p}", &i2);
    println!("Stack address of i3 is : {:p}", &i3);

    let y = 5;
    println!("Stack address of y is : {:p}", &y);
    let y = y + 1;
    println!("Stack address of y is : {:p}", &y);
    let y = y + 2;
    println!("Stack address of y is : {:p}", &y);
    println!("Y value is {}", y);
    {
        let y = 0;
        println!("The value of y is {}", y);
    }
    println!("The value of y is {}", y);

    let t1 = (500, 6.0, "dummy");
    let (_x, _y, _z) = t1;
    println!("The value of t1 is : {} {} {}", t1.0, t1.1, t1.2);

    // tupleはいつものC/C++の使い方と同じ
    let mut t2 = ((0, 1), (2, 3));
    let ((ref mut x1_ptr, ref mut y1_ptr), _) = t2;
    *x1_ptr = 5;
    *y1_ptr = -5;
    println!("{:?}", t2);

    let a1 = [1, 2, 3, 4, 5];
    let a2 = [0; 10];
    println!("{:?} {:?} {} {}", a1, a2, a1[2], a2[3]);

    // let s1 = "Helloこんにちは挨拶";
    // let s2 = "hello";
    // println!("Stack address of s1 {:p}", &s1);
    // println!("Stack address of s2 {:p}", &s2);
    // println!("Static memory address of s1: {:?}", s1.as_ptr());
    // println!("Static memory address of s2: {:?}", s2.as_ptr());
    // println!("Len of s1 is: {}", s1.len());
    // println!("Len of s2 is: {}", s2.len());

    let mut s1 = String::from("Hello");
    let mut s2 = String::from("HelloWorld");

    println!("Stack address of s1 {:p}", &s1);
    println!("Stack address of s2 {:p}", &s2);
    println!("Heap address of s1 {:?}", s1.as_ptr());
    println!("Heap address of s2 {:?}", s2.as_ptr());
    println!("Cap of s1 : {}", s1.capacity());
    println!("Len of s1 : {}", s1.len());
    println!("Cap of s2 : {}", s2.capacity());
    println!("Len of s2 : {}", s2.len());
    s1.push_str("_new1");
    s2.push_str("_new2");
    println!("{} {}", s1, s2)

    
}
