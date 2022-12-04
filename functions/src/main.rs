fn main() {
    println!("Hello, world!");
    another_function();
    parameterized_function(5);

    let x = plus_one(5);
    println!("The value of x is: {x}");
}

fn another_function() {
    println!("Hi I'm the other function")
}

fn parameterized_function(x: u32) {
    println!("Hi I'm the other function with param value is {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}