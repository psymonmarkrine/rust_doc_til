fn main() {
    let mut s = String::from("THE QUICK BROWN FOX");
    let word = first_word(&s);
    
    let t = "JUMPS OVER THE LAZY DOG";

    println!("{}, {}", s, word);

    let word = first_word(&t);

    s.clear();

    println!("{}, {}", s, word);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}