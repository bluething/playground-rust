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
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
}