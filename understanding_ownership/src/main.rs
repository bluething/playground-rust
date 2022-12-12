#![allow(unused)]

fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    //println!("{}, world!", s1);

    let s3 = String::from("hello");
    let s4 = s3.clone();

    println!("s3 = {}, s4 = {}", s3, s4);
}
