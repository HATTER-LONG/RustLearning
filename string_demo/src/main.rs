fn main() {
    let data = "initial contents";
    let _s = data.to_string();

    let _s1 = "initial contents".to_string();
    test_push_str();
}

fn test_push_str() {
    let mut s = String::from("foo");
    s.push_str("bar"); //foobar

    let s1 = "bar2".to_string();
    s.push_str(&s1); // 仅借用 s1 不会获取其所有权，后续可继续使用

    s.push('l');
    test_plus_str();
}

fn test_plus_str() {
    let s1 = "Hello, ".to_string();
    let s2 = "World!".to_string();

    let s3 = s1 + &s2; // 这里 s1 需要是 String，s2 是要求字符串切片或 String 类型引用。

    /*
    println!("s1 = {}", s1);  s1 已经被 moved，因此不能再使用
    error[E0382]: borrow of moved value: `s1`
      --> src/main.rs:25:25
       |
    20 |     let s1 = "Hello, ".to_string();
       |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
    ...
    23 |     let s3 = s1 + &s2; // 这里 s1 需要是 String，s2 是要求字符串切片或 String 类型引用。
       |              -- value moved here
    24 |
    25 |     println!("s1 = {}", s1);
       |                         ^^ value borrowed here after move
      */
    println!("s2 = {}", s2);
    println!("s3 = {}", s3);
    test_fomat();
}

fn test_fomat() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    //let s3 = s1 + "-" + &s2 + "-" + &s3;
    //print!("{}", s3);

    let s = format!("{}-{}-{}", s1, s2, s3);
    print!("{}", s);
    test_len();
}

fn test_len() {
    let len = String::from("hola").len();
    println!("{}", len);
    test_split();
}

fn test_split() {
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{}", s); //Зд

    /*
    thread 'main' panicked at 'byte index 3 is not a char boundary; it is inside 'д' (bytes 2..4) of `Здравствуйте`'
    , src/main.rs:69:14
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    */
    let s = &hello[0..3];
}
