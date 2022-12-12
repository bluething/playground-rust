#![allow(unused)]

fn main() {
    let mut original: String = String::from("original value");
    println!("\noriginal: \t\"{}\"\n", original);
    original = String::from("new value");

    let next = &original;
    println!("\noriginal: \t\"{}\"\n", original);
}
