fn main() {
    let s1 = String::from("foo");
    let s2 = String::from("bar");
    let s3 = s1 + &s2;

    let s = format!("{}-{}", s2, s3);

    println!("{}", s2);
    println!("{}", s3);
    println!("{}", s);
}
