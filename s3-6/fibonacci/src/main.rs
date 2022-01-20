use std::io;

fn main() {
    loop {
        let mut num = String::new();   

        println!("Number of Fibonacci sequence: ");
        io::stdin().read_line(&mut num)
            .expect("Failed to read line");
        
        let num: u32 = match num.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };

        let mut sum = (0, 1);
        for _ in 0..num {
            sum = (sum.0 + sum.1, sum.0);
        }

        println!("The {}-th Fibonacci number is: {}", num, sum.0);
        break;
    }
}
