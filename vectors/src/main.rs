fn main() {
    let v: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];

    let v3 = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v3[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v3.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let mut v4 = vec![1, 2, 3, 4, 5];

    let first = &v4[0];

    v4.push(6);

    println!("The first element is: {first}");

}
