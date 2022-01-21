fn main() {
    let s1 = give_ownership();
    let s2 = String::from("hello");
    let s3 = take_and_give_back(s2);

    println!("s1: {}", s1);
    // println!("s2: {}", s2);
    println!("s3: {}", s3);
}

fn give_ownership() -> String {
    String::from("HALO!!!!")
}

fn take_and_give_back(a_string: String) -> String {
    a_string
}