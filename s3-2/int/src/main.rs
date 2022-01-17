fn main() {
    let x = 98_222;
    println!("x: {}", x);
    let x = 0xff;
    println!("x: {}", x);
    let x = 0o77;
    println!("x: {}", x);
    let x = 0b1111_0000;
    println!("x: {}", x);
    let x = b'A';
    println!("x: {}", x);
    // xはu32として解釈された
}
