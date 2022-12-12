#![allow(unused)]

fn main() {
    let mut original: String = String::from("original value");
    println!("\noriginal: \t\"{}\"\n", original);

    {
        let next = &mut original;
        *next = String::from("next value");
        println!("inner scope original: \t\"{}\"\n", original);
        println!("inner scope next: \t\"{}\"\n", next);
    }

    println!("outer scope original: \t\"{}\"\n", original);
}
