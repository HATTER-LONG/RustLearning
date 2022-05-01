use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("猜数游戏！");

    let secret_number = rand::thread_rng().gen_range(1..101); // i32 u32 i64
    println!("随机数为: {}", secret_number);

    loop {
        println!("猜测一个数:");

        let mut guess = String::new();

        // 1.  & 引用 默认是不可变，需要加上 mut 变为可变引用
        // 2.  Result 是一种返回的枚举类型，IO 有 OK ERR 两个枚举。
        //      - expect 方法也是 Result 中的一个方法，当返回 ERR 时会调用 expect 返回传入的字符串信息。
        //      - 当返回 OK 是，则正常返回结果给调用。
        io::stdin().read_line(&mut guess).expect("无法读取行");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你猜测的数字是：{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
