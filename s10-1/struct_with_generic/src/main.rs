// https://docs.julialang.org/en/v1/manual/types/#man-parametric-composite-types
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }

    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let int_and_float = Point { x: 2, y: 3.0 };

    println!("integer.x = {}", integer.x());
    println!("float.x = {}", float.x());
    println!("int_and_float.x = {}", int_and_float.x());

    let mix = integer.mixup(float);
    println!("mix.x = {}, mix.y = {}", mix.x, mix.y);
}
