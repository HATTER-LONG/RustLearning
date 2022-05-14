fn main() {
    let mut s = String::from("Hello");
    s.push_str(", World");
    println!("{}", s);

    //let s2 = s;

    let s2 = s.clone();
    println!("s = {}", s);
    test()
}
fn test() {
    let s = String::from("Hello, World");
    take_ownership(s);
    //println!("{}", s); err
    let x = 5;
    make_copy(x);
    println!("{}", x);
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn make_copy(some_number: i32) {
    println!("{}", some_number);
    test2()
}

fn test2() {
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);
    test3()
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn test3() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
    test4()
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
fn test4() {
    let mut s1 = String::from("hello");
    let len = calculate_length_and_append(&mut s1);

    println!("The length of '{}' is {}.", s1, len);

    let s2 = &mut s1;
    //let s3 = &mut s1;
    println!("The length of {} {}", s2, s3);
}

fn calculate_length_and_append(s: &mut String) -> usize {
    s.push_str(", world");
    s.len()
}

fn test5() {
    let mut s = String::from("hello");
    {
        let s1 = &mut s;
    }

    let s2 = &mut s;
}

fn test6() {
    let r = dangle();
}
fn dangle() -> &String {
    let s = String::from("Hello");
    &s
}
