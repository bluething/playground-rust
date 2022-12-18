#![allow(unused)]

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    println!("{}", some_number.unwrap());
    //println!("{}", absent_number.unwrap());     // casue panic
}

enum IpAddrKind {
    V4,
    V6,
}
