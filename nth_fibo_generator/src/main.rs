use std::io;

fn main() {
    println!("How many fibo do you want?");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let counter: u32 = input.trim().parse().unwrap();
    for i in 0..counter {
        println!("The {i} fibo is {}", nth_fibo(i));
    }

}

fn nth_fibo(n: u32) -> i32 {
    let mut a = 0;
    let mut b = 1;
    let mut c;
    if n == 0 {
        return 0
    }
    for _i in 2..=n {
        c = a + b;
        a = b;
        b = c;
    }

    b
}
