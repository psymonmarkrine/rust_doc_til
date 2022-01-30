
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }
// 
// fn value_in_cents(coin: Coin) -> u32 {
//     match coin {
//         Coin::Penny => {
//             println!("Lucky penny!");
//             1
//         },
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }

// ################################

// #[derive(Debug)]
// enum UsState {
//     Alabama,
//     Alaska,
//     // ... etc.
// }
// 
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }
// 
// fn value_in_cents(coin: Coin) -> u32 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {:?}!", state);
//             25
//         },
//     }
// }

// ################################

// let some_u8_value = 0u8;
// match some_u8_value {
//     1 => println!("one"),
//     3 => println!("three"),
//     5 => println!("five"),
//     7 => println!("seven"),
//     _ => (),
// }
