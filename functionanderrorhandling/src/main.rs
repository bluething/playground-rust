#![allow(unused)]
use std::fs::File;
use std::io::ErrorKind;

use std::io::Stderr;

fn main() {
    let greater = return_greater(10, 5);
    println!("{}", greater);

    let mut original = String::from("original value");
    println!("\nouter scope original: \t\"{}\"", original);

    {
        print_original(&original);
        change_original(&mut original);
        println!("inner scope original: \t\"{}\"", original);
    }

    println!("outer scope original: \t\"{}\"", original);

    // closure
    write_message();

    let name = "Duck Airlines";

    let write_message_closure = |slogan: String| -> String {
        String::from(format!("{}. {}", name, slogan))
    };

    let phrase = write_message_closure(String::from("We hit the ground every time"));
    println!("{}", phrase);

    //panic_vector();     // cause panic because index out of bound

    let filename = "customer.json";
    match File::open(filename) {
        Ok(file) => {
            println!("{:#?}", file);
        }
        Err(error) => {
            match error.kind() {
                ErrorKind::NotFound => {
                    match File::create(filename) {
                        Ok(file) => {
                            println!("File created");
                        }
                        Err(error) => {
                            println!("{:#?}", error);
                        }
                    }
                }
                _ => {
                    println!("{:#?}", error);
                }
            }
        }
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

fn change_original(original: &mut String) {
    let next = original;
    *next = String::from("next value");
    println!("fn change_original: \t\"{}\"", next);     // use next because original no longer own the memory
}

fn write_message() {
    let name = "Duck Airlines";
    let slogan = "We hit the ground every time";
    println!("Welcome to {}. {}", name, slogan);
}

fn panic_vector() {
    let vector = vec![1, 2, 3, 4, 5];
    println!("{}", vector[10]);
}
