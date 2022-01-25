struct Color(u8, u8, u8);
struct Point(u8, u8, u8);
struct UnitLike();

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(black.0, 0, 0);
    let unit = UnitLike();

    let Color(a, b, c) = black;
    println!("{}", a);
}
