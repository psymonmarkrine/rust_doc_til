fn main() {
    // loop {
    //     println!("again!");
    // }

    // let mut number = 3;
    // 
    // while number != 0 {
    //     println!("{}?", number);
    // 
    //     number = number - 1;
    // }
    // 
    // println!("Launch!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);

        index = index + 1;
    }
}
