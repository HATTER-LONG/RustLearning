fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3 or 2");
    }

    match number % 4 {
        0 => println!("number is divisible by 4"),
        _ => println!("number is notdivisible by 4"),
    };
    ifvar();
}

fn ifvar() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is : {}", number);

    loop_f();
}

fn loop_f() {
    let mut count = 0;
    let result = loop {
        count = count + 1;
        println!("loop count : {}", count);

        if count == 10 {
            break count * 2;
        }
    };
    while_f();
}

fn while_f() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }

    println!("LIFTOFF!!!!");
    for_f()
}

fn for_f() {
    let a = [10, 20, 30, 40, 50];
    // iter 迭代器
    for element in a.iter() {
        println!("the value is : {}", element);
    }
    for_range();
}

fn for_range() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
