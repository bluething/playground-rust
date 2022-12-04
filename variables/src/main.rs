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
}
