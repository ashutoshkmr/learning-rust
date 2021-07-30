struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn main() {
    let mut user1 = build_user(
        String::from("ashutoshkmr40@gmail.com"),
        String::from("Ashutosh Kumar"),
    );

    user1.email = String::from("someemail@example.com");

    let user2 = User {
        username: String::from("Some username"),
        email: String::from("anotheremail@example.com"),
        ..user1
    };

    println!("{}", user2.active)
}

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}
