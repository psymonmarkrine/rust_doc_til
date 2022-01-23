fn main() {

    let s1 = String::from("THE_QUICK_BROWN_FOX");

    let len = calclate_length(&s1);

    println!("The length of \"{}\" is {}.", s1, len);
}

fn calclate_length(s: &String) -> usize {
    // s.push_str("_JUMPS_OVER_THE_LAZY_DOG");
    s.len()
}
