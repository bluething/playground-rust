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

    let c = 'z';
    println!("The char is {c}");
    let z: char = 'ℤ'; // with explicit type annotation
    println!("The char is {z}");
    let heart_eyed_cat = '😻';
    println!("The char is {heart_eyed_cat}");

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");
    let first_element = tup.0;
    println!("The value of first tuple is: {first_element}")
}
