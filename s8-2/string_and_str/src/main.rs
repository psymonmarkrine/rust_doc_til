fn main() {
    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2);
    s.push('!');
    
    println!("{}", s);
}
