const MAX_POINTS: u32 = 100_000;

fn main() {
    println!("Hello, world!");

    let x = 5;
    println!("The value of x is {}", x);
    //x = 6; error
    let x = 6; //shadowing
    println!("The value of x is {}", x);

    let space = "    "; // str
    let space = space.len(); // usize
    println!("now space value is {}", space);

    let parse_num: u32 = "42".parse().expect("Not a number");
    println!("parse number is {}", parse_num);

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("{},{},{}", tup.0, tup.1, tup.2);

    let (x, y, z) = tup;
    println!("x={},y={},z={}", x, y, z);

    let a = [1, 2, 3, 4, 5];
    println!("a[0] = {}, a[2] = {}", a[0], a[2]);
}
