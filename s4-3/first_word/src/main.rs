fn main() {
    let mut s = String::from("THE QUICK BROWN FOX");
    let word = first_word(&s);

    println!("{}, {}", s, word);

    // s.clear();

    // println!("{}, {}", s, word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}