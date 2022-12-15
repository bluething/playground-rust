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

    let s5 = gives_ownership();         // gives_ownership moves its return
    // value into s5

    let s6 = String::from("hello");     // s6 comes into scope

    let s7 = takes_and_gives_back(s6);  // s6 is moved into
    // takes_and_gives_back, which also
    // moves its return value into s7
    //println!("{}", s6);                       //borrow of moved value: s6
    println!("{}", s7);

    let s8 = String::from("hello");
    let (s9, len) = calculate_length(s8);
    println!("The length of '{}' is {}.", s9, len);

    let s10 = String::from("hello");
    let len = calculate_length_2(&s10);
    println!("The length of '{}' is {}.", s10, len);

    let mut s11 = String::from("hello");
    change(&mut s11);

    let mut s12 = String::from("hello");

    let r1 = &mut s12;
    let r2 = &mut s12;

    //println!("{}, {}", r1, r2);

    let mut s13 = String::from("hello");

    {
        let r1 = &mut s13;
        println!("r1: {}", r1);
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s13;
    println!("r2: {}", r2);

    let mut s14 = String::from("hello");

    let r3 = &s14; // no problem
    let r4 = &s14; // no problem
    //let r5 = &mut s14; // BIG PROBLEM

    let mut s15 = String::from("hello");

    let r6 = &s15; // no problem
    let r7 = &s15; // no problem
    println!("{} and {}", r6, r7);
    // variables r6 and r7 will not be used after this point

    let r8 = &mut s15; // no problem
    println!("r8: {}", r8);

    //let reference_to_nothing = dangle();
    let reference_to_thing = no_dangle();

    let mut s16 = String::from("hello world");

    let word = first_word(&s16); // word will get the value 5

    s16.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}
// Here, x goes out of scope, then s. But because s's value was moved, nothing
// special happens.
//
// Here, s7 goes out of scope and is dropped. s6 was moved, so nothing
// happens. s5 goes out of scope and is dropped.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
}

fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn calculate_length_2(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

//fn dangle() -> &String {                        // dangle returns a reference to a String
//    let s = String::from("hello");     // s is a new String
//
//    &s                                          // we return a reference to the String, s
//}
// Here, s goes out of scope, and is dropped. Its memory goes away.
// Danger!

fn no_dangle() -> String {                        // return the String
    let s = String::from("hello");     // s is a new String

    s                                      // return the Sting directly
}
// Here, s goes out of scope, and is dropped. Its memory goes away.

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}