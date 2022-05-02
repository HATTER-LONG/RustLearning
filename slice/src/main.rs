fn main() {
    let s = String::from("hello world");

    let hello = &s[..5];
    let world = &s[6..];

    let whole = &s[..];
    println!("hello = {}, world = {}", hello, world);
    let word_index = first_world(&s);

    println!("{}", word_index);

    let word_index = first_world_v2(&s);
}

fn first_world(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_world_v2(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
