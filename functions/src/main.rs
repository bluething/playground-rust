fn main() {
    println!("Hello, world!");
    another_function();
    parameterized_function(5);
}

fn another_function() {
    println!("Hi I'm the other function")
}

fn parameterized_function(x: u32) {
    println!("Hi I'm the other function with param value is {x}");
}
