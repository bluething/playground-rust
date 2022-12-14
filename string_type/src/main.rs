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

    let mut s5 = String::from("foo");
    s5.push_str("bar");
    println!("{}", s5);

    let mut s6 = String::from("foo");
    let s7 = "bar";
    s6.push_str(s7);
    println!("s7 is {s7}");

    let mut s8 = String::from("lo");
    s8.push('l');
    println!("{}", s8);

    let s9 = String::from("Hello, ");
    let s10 = String::from("world!");
    let s11 = s9 + &s10; // note s9 has been moved here and can no longer be used
    println!("{}", s11);

    let s12 = String::from("tic");
    let s13 = String::from("tac");
    let s14 = String::from("toe");

    let s15 = format!("{s12}-{s13}-{s14}");
    println!("{}", s15);

    let s15 = String::from("hello");
    //let h = s15[0];

    let hello = String::from("Здравствуйте");
    //let answer = &hello[0];

    for c in "Зд".chars() {
        println!("{c}");
    }
    for b in "Зд".bytes() {
        println!("{b}");
    }

}
