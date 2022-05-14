use std::fs::File;
use std::io::{ErrorKind, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    main2();
    let f = File::open("hello.txt")?;
    Ok(())
}

fn main2() {
    let name = read_username_from_file();
    println!("name is {}", name.unwrap());

    let f = File::open("hello.txt");
    let _file = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error creating file: {:?}", e),
            },
            other_error => panic!("Error opening the file : {:?}", other_error),
        },
    };
    closure_test();
}

fn closure_test() {
    let _f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Error creating file : {:?}", error);
            })
        } else {
            panic!("Error opening file : {:?}", error);
        }
    });
    unwrap_test();
    expect_test();
}

fn unwrap_test() {
    let _f = File::open("hello.txt").unwrap();
}

fn expect_test() {
    let _f = File::open("hello.txt").expect("无法打开文件");
}

fn read_username_from_file() -> Result<String, std::io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_by_special() -> Result<String, std::io::Error> {
    let mut f = File::open("hello.txt")?;

    let mut s = String::new();

    f.read_to_string(&mut s)?;
    Ok(s)
}

fn read_username_from_file_by_special2() -> Result<String, std::io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if !(1..=100).contains(&value) {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
