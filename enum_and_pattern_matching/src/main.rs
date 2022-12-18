#![allow(unused)]

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    println!("{}", some_number.unwrap());
    //println!("{}", absent_number.unwrap());     // casue panic

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    //let sum = x + y;                          // the trait `Add<Option<i8>>` is not implemented for `i8`

    let coin_penny = value_in_cents(Coin::Penny);
    println!("{}", coin_penny);

    let coin_quarter_alabama = value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("{}", coin_quarter_alabama);
}

enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);        // 'UsState' cannot be formatted using '{:?}'

            25
        },
    }
}