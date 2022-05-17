use trait_demo::Summary;
use trait_demo::Tweet;

fn largest<T: PartialOrd + Clone>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn check() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![String::from("Hello"), String::from("World")];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
}

fn main() {
    check();
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably alread know, people"),
        reply: false,
        retweet: false,
    };

    println!("one new tweet: {}", tweet.summarize())
}
