#![allow(unused)]

fn main() {
    let mut original: String = String::from("original value");
    println!("\noriginal: \t\"{}\"\n", original);

    {
        let next = &mut original;
        *next = String::from("next value");
        println!("inner scope next: \t\"{}\"\n", next);
        println!("inner scope original: \t\"{}\"\n", original);
    }

    println!("outer scope original: \t\"{}\"\n", original);

    let outer_scope: &i32;
    {
        let inner_scope = 5;
        outer_scope = &inner_scope;
    }

    //println!("{}", outer_scope);

    //let returned_ref = return_bad_ref();

    let reference_int = 6;
    let returned_value = return_one_parameter(&reference_int);
    println!("{}", returned_value);

    let value_one = 24;
    let value_two = 67;
    let value: &i32 = explicit_lifetime(&value_one, &value_two);
}

//fn return_bad_ref() -> &i32 {
//    let value: i32 = 5;
//    &value
//}

fn lifetime_syntax<'a, 'b>(p1: &'a i32, p2: i32, p3: &'b  f64) {

}

fn return_one_parameter(value: &i32) -> &i32 {
    value
}

fn explicit_lifetime<'a>(p1: &'a i32, p2: &'a i32) -> &'a i32 {
    if p1 > p2 {
        p1
    } else {
        p2
    }
}
