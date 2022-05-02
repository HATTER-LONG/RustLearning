struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let _user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user2.email = String::from("aaaa@example.com");

    let user_info: User = test("aaa@ex.com".to_string(), "ccl".to_string());
    let _user_info2: User = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername"),
        active: user_info.active,
        sign_in_count: user_info.sign_in_count,
    };
    let _user_info3: User = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername"),
        ..user_info
    };

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let origin = Point(0,0,0);
}

fn test(email: String, username: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}
