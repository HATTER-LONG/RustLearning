fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);

    println!("The longest string is {}", result);
    test_struct_lifttime();
}

// longest 的返回值会是 x、y 其中之一，x、y 的具体生命周期也无法确定
// 而且与函数的实现体无关，rust 只检查函数签名
// 需要手动增加生命周期。
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    // 返回的引用被赋予来 self 的生命周期
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn test_struct_lifttime() {
    let novel = String::from("Call me Ishmael. Some years ago ...");
    let first_sentence = novel.split('.').next().expect("Could not found a .");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("print part {}", i.part);
    longest_with_an_announcement("aa", "bb", "cc");
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: std::fmt::Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
