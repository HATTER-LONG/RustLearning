pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {} ...)", self.summarize_author())
    }
}

pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewArticle {
    // fn summarize(&self) -> String {
    //     format!("{}, by {} ({})", self.headline, self.author, self.location)
    // }
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_bound<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_mix(item: impl Summary + std::fmt::Display) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_bound_mix<T: Summary + std::fmt::Display>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify_bound_mulit_mix<T: Summary + std::fmt::Display, U: Clone + core::fmt::Debug>(
    a: T,
    _b: U,
) -> String {
    format!("Breaking news! {}", a.summarize())
}

pub fn notify_bound_mulit_mix_where<T, U>(a: T, _b: U) -> String
where
    T: Summary + std::fmt::Display,
    U: Clone + core::fmt::Debug,
{
    format!("Breaking news! {}", a.summarize())
}

pub fn notify_return_trait(s: &str) -> impl Summary {
    NewArticle {
        headline: String::from("headline str"),
        content: String::from("content text"),
        author: String::from("author info"),
        location: String::from("location info"),
    }
    // 接口中只能返回一种 Trait，类似继承的派生类不会强转为父类
    // if(xxx)
    // {
    //     return tweetinfo will error
    // }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: std::fmt::Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is x = {}", self.y);
        }
    }
}
