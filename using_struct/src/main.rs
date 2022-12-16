#![allow(unused)]

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

    let user3 = build_user(String::from("fulan@example.com"), String::from("Fulan"));
    println!("User name: {}", user3.username);

    let user4 = build_user_using_init_shorthand(String::from("fulan@example.com"), String::from("Fulan"));
    println!("User email: {}", user4.email);

    let user5 = User {
        email: String::from("fulanah@example.com"),
        username: user1.username,
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    println!("User email: {}", user5.email);

    let user6 = User {
        email: String::from("fulanah@example.com"),
        ..user2
    };
    println!("User email: {}", user6.email);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black0: {}, black1: {}, black3: {}", black.0, black.1, black.2);
    println!("origin0: {}, origin1: {}, origin3: {}", origin.0, origin.1, origin.2);

    let subject = AlwaysEqual;

    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn build_user_using_init_shorthand(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}