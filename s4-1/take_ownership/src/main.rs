fn main() {
    let s = String::from("hello?");
    let x = 5;

    println!("{}", x);
    println!("{}", s);

    println!("throw to fn");

    makes_copy(x);
    // take_ownership(s);

    println!("{}", x);
    println!("{}", s);
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}