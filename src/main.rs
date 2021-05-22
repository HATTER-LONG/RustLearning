fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("Hello World {} {} {}", c, z, heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    //let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    println!(
        "x = {}|{}, y = {}|{}, z = {}",
        x, five_hundred, y, six_point_four, z
    );

    let a: [i32; 6] = [1, 2, 3, 4, 5, 6];
    //let a = [1, 2, 3, 4, 5, 6];
    let b = [3; 5]; //[3, 3, 3, 3, 3]
    println!("a = {}, b[2] = {}", a[0], b[2]);

    let res = another_function(33, 22);

    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("x = {} y = {} res = {}", x, y, res);
}

fn another_function(x: i32, y: i32) -> i32 {
    println!("The value of x is : {} y is : {}", x, y);
    let x = 5;
    x
}
