#![allow(unused)]

fn main() {
    let mut original: String = String::from("original value");
    println!("\noriginal: \t\"{}\"\n", original);

    let next = &original;
    original = String::from("new value");
    println!("\noriginal: \t\"{}\"\n", original);
}
