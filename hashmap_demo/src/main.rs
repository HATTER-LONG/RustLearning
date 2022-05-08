use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    test_collect();
}

fn test_collect() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let intial_scores = vec![10, 50];

    // 使用 zip （拉链）来创建一个 tuple
    let _scores: HashMap<_, _> = teams.iter().zip(intial_scores.iter()).collect();
    test_owner();
}

fn test_owner() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    /*
        error[E0382]: borrow of moved value: `field_name`
    --> src/main.rs:24:24
    |
    18 |     let field_name = String::from("Favorite color");
    |         ---------- move occurs because `field_name` has type `String`, which does not implement the `Copy`
    trait
    ...
    22 |     map.insert(field_name, field_value);
    |                ---------- value moved here
    23 |
    24 |     println!("{}: {}", field_name, field_value);
    |                        ^^^^^^^^^^ value borrowed here after move
    |
    */
    //println!("{}: {}", field_name, field_value);
    test_owner2();
}

fn test_owner2() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);

    println!("{}: {}", field_name, field_value);

    test_get();
}

fn test_get() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");

    let score = scores.get(&team_name);

    match score {
        Some(s) => println!("{}", s),
        None => println!("team not exit"),
    }
    test_for();
}

fn test_for() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (k, v) in &scores {
        println!("{}: {}", k, v);
    }
    test_replace();
}

fn test_replace() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 50);

    println!("{:?}", scores); //{"Blue": 50}
    test_check_and_insert();
}

fn test_check_and_insert() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    //scores.entry(String::from("Yellow")).or_insert(50);
    let e = scores.entry(String::from("Yellow"));
    println!("{:?}", e); //Entry(VacantEntry("Yellow"))

    e.or_insert(50);

    let e = scores.entry(String::from("Blue"));
    println!("{:?}", e); //Entry(OccupiedEntry { key: "Blue", value: 10, .. })
    e.or_insert(50);

    println!("{:?}", scores); //{"Blue": 10, "Yellow": 50}
    test_update_depend_current();
}

fn test_update_depend_current() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", map); //{"wonderful": 1, "world": 2, "hello": 1}
}
