fn main() {
    let _some_number = Some(5);
    let _some_string = Some("A String");

    let _absent_number: Option<i32> = None;

    test_option()
}

fn test_option() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y;
    let c = Coin::Quarter(UsState::Alaska);
    test_match(c);
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn test_match(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn call() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main2() {
    let v = 0_u8;
    match v {
        1 => println!("one"),
        3 => println!("three"),
        _ => (),
    }

    let v = Some(0_u8);
    match v {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = v {
        println!("three");
    } else{
        
        println!("other");
    }
}
