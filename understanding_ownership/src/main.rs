#![allow(unused)]

fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    //println!("{}, world!", s1);

    let s3 = String::from("hello");
    let s4 = s3.clone();

    println!("s3 = {}, s4 = {}", s3, s4);

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    let s = String::from("hello");    // s comes into scope

    takes_ownership(s);              // s's value moves into the function...
    //println!("{}", s);                        // ... and so is no longer valid here

    let x = 5;                             // x comes into scope

    makes_copy(x);                  // x would move into the function,
    // but i32 is Copy, so it's okay to still
    // use x afterward

    let s5 = gives_ownership();         // gives_ownership moves its return
    // value into s5

    let s6 = String::from("hello");     // s6 comes into scope

    let s7 = takes_and_gives_back(s6);  // s6 is moved into
    // takes_and_gives_back, which also
    // moves its return value into s7
    //println!("{}", s6);                       //borrow of moved value: s6
    println!("{}", s7);

    let s8 = String::from("hello");
    let (s9, len) = calculate_length(s8);
    println!("The length of '{}' is {}.", s9, len);

    let s10 = String::from("hello");
    let len = calculate_length_2(&s10);
    println!("The length of '{}' is {}.", s10, len);

    let mut s11 = String::from("hello");
    change(&mut s11);
}
// Here, x goes out of scope, then s. But because s's value was moved, nothing
// special happens.
//
// Here, s7 goes out of scope and is dropped. s6 was moved, so nothing
// happens. s5 goes out of scope and is dropped.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
}

fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn calculate_length_2(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}