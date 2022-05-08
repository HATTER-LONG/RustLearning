fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];

    println!("The third element is {}", third);

    match v.get(2) {
        Some(num) => println!("The third element is {}", num),
        None => println!("There is no third element"),
    }
    test();
}

fn test() {
    let mut v = vec![1, 2, 3, 4, 5];
    let third = &v[2];
    //v.push(6); // 这时存在对 v 对可变与不可变借用（third）会报错

    println!("The third element is {}", third);

    v.push(6); // 这时不可变应用 third 后续没有再进行使用，因此可以正常的可变借用
    test_for();
}

fn test_for() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        println!("{}", i);
        *i += 50;
    }
    for i in v {
        println!("{}", i);
    }
    test_vector_enum();
}

enum SpreadssheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn test_vector_enum() {
    let _row = vec![
        SpreadssheetCell::Int(3),
        SpreadssheetCell::Text(String::from("blue")),
        SpreadssheetCell::Float(10.12),
    ];
    
}
