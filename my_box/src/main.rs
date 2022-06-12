use std::ops::Deref;

fn hello(name: &str) {
    println!("Hello, {}", name);
}

fn main() {
    let m = MyBox::new(String::from("Rust"));

    // &MyBox<String> deref  &String
    // &String deref &str
    hello(&m);

    // 如果没有 deref
    hello(&(*m)[..]);

    hello("Rust");
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    /// Creates a new [`MyBox<T>`].
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
