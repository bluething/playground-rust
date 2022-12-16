fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("User email: {}", user1.email);

    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user2.email = String::from("fulan@example.com");
    println!("User email: {}", user2.email);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}