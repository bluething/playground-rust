fn main() {
    let mut s = String::new();
    println!("{}", s);

    let data = "initial contents";
    println!("{}", data);

    let s2 = data.to_string();
    println!("{}", s2);

    // the method also works on a literal directly:
    let s3 = "initial contents".to_string();
    println!("{}", s3);

    let s4 = String::from("initial contents");
    println!("{}", s4);
}
