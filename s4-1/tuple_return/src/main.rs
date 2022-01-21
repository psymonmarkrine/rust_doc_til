fn main() {
    let s = String::from("THE_QUICK_BROWN_FOX");
    let (s, len) = caluculate_length(s);

    println!("The length of \"{}\" is {}.", s, len);
}

fn caluculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}