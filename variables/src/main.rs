fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds is: {THREE_HOURS_IN_SECONDS}");

    let y: u8 = 5;
    println!("The value of y is {y}");
    let y: u8 = 6;
    println!("The value of y is {y}");
    {
        let y: f32 = 10.0;
        println!("The value of y is {:.1}", y);
    }
    println!("The value of y is {y}");

    // addition
    let sum = 5 + 10;
    println!("The results is {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("The results is {:.1}", difference);

    // multiplication
    let product = 4 * 30;
    println!("The results is {product}");

    // division
    let quotient = 56.7 / 32.2;
    println!("The results is {:.2}", quotient);
    let floored = 2 / 3; // Results in 0
    println!("The results is {:.1}", floored);

    // remainder
    let remainder = 43 % 5;
    println!("The results is {remainder}");
}
