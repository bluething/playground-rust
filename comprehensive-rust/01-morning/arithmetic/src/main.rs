fn interprodcut(a: i32, b: i32, c: i32) -> i32 {
    return a * b + b * c + c * a;
}

fn main() {
    println!("result: {}", interprodcut(120, 100, 248));
}
