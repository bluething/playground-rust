#![allow(unused)]

fn main() {
    let mut original: String = String::from("original value");
    println!("\noriginal: \t\"{}\"\n", original);

    {
        let next = &mut original;
        *next = String::from("next value");
        println!("inner scope next: \t\"{}\"\n", next);
        println!("inner scope original: \t\"{}\"\n", original);
    }

    println!("outer scope original: \t\"{}\"\n", original);

    let outer_scope: &i32;
    {
        let inner_scope = 5;
        outer_scope = &inner_scope;
    }

    println!("{}", outer_scope);
}
