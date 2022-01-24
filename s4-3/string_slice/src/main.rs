fn main() {
    let s = String::from("THE QUICK BROWN FOX");

    let the = &s[..3];
    let quick = &s[4..9];
    let brown = &s[10..15];
    let fox = &s[16..];
    
    println!("{} {} {} {}", the ,quick, brown, fox);

    println!("{}", &s[..])
}
