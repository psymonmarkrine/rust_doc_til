// https://docs.julialang.org/en/v1/manual/types/#man-parametric-composite-types
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let int_and_float = Point { x: 2, y: 3.0 };
}
