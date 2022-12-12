#![allow(unused)]

fn main() {
    let original: String = String::from("original value");
    println!("\noriginal: \t\"{}\"\n", original);

    let next = &original;
    println!("\noriginal: \t\"{}\"\n", original);
}
