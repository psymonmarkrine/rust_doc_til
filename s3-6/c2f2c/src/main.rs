use std::io;

fn main() {
    let mut cf = String::new();

    loop {
        println!("to Fahrenheit or Celsius (F/c):"); // https://qiita.com/V_lasergun/items/bb2483f3ce6bd377c863
        io::stdin()
            .read_line(&mut cf)
            .expect("Failed to read line");

        let cf: char = match cf.trim().to_lowercase().to_string().as_str() {
            "f" => 'f',
            "c" => 'c',
            _ => continue,
        };

        cf_convert(cf);
        break;
    }
}

fn cf_convert(cf: char) {
    let mut temp = String::new();

    loop {
        println!("Temperature: ");
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        let temp: f64 = match temp.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };

        match cf {
            'f' => println!("Celsius: {}, Fahrenheit: {:.2}", temp, temp * 9. / 5. + 32.),
            'c' => println!("Fahrenheit: {}, Celsius: {:.2}", temp, (temp - 32.) * 5. / 9.),
            _ => println!("Error: unable to solve"),
        }

        break;
    }
}