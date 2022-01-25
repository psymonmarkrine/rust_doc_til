struct User {
    email: String,
    username: String,
    active: bool,
    sign_in_count: usize,
}

fn main() {
    let user1 = User {
        email: String::from("itsme@domein.net"),
        username: String::from("psymonmarkrine"),
        sign_in_count: 10,
        active: true,
    };

    println!("{}", user1.username);

    let user2 = build_user(String::from("going@how.it.is"), String::from("psycojitwheet"));

    println!("{}", user2.email);

    let user3 = User {
        sign_in_count: 10,
        ..user2
    };
    println!("{}", user3.username)
}

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        active: false,
        sign_in_count: 0
    }
}