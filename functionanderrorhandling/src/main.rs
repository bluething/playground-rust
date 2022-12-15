#![allow(unused)]

fn main() {
    let greater = return_greater(10, 5);
    println!("{}", greater);

    let mut original = String::from("original value");
    println!("\nouter scope original: \t\"{}\"", original);
    {
        print_original(&original);
    }
}

fn return_greater(first: u8, second: u8) -> u8 {
    if first > second {
        first
    } else {
        second
    }
}

fn print_original(original: &String) {
    println!("fn print_original: \t\"{}\"", original);
}
