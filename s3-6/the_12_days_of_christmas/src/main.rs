fn main() {
    const TOTAL_DAY: u32 = 12;

    for day in 1..TOTAL_DAY+1 {
        match day {
            1 => println!("On the first day of Christmas,"),
            2 => println!("On the second day of Christmas,"),
            3 => println!("On the third day of Christmas,"),
            num => println!("On the {}-th day of Christmas,", num),
        };
        
        println!("my true love sent to me");

        for n in (0..day+1).rev() {
            match n {
                1 => println!("a partrudge in a pear tree."),
                2 => {
                    println!("two turtle doves,");
                    print!("and ");
                },
                3 => println!("three French hens,"),
                4 => println!("four calling birds,"),
                5 => println!("five golden rings."),
                6 => println!("six geese a-laying,"),
                7 => println!("seven swans a-swimming,"),
                8 => println!("eight maids a-milking,"),
                9 => println!("nine ladies dancing,"),
                10 => println!("ten lords a-leaping,"),
                11 => println!("eleven pipers piping,"),
                12 => println!("twelve drummers drumming,"),
                _ => println!(""),
            };
        } // end for days.rev()
    } // end for TOTAL_DAY
}
