use std::io;

fn main() {
    loop {
        println!("Choose a conversion:");
        println!("1. Fahrenheit to Celsius");
        println!("2. Celsius to Fahrenheit");

        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Error in option");

        let option: i32 = match option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That is not a correct option type");
                continue;
            },
        };

        println!("What is your temperature?");

        let mut temp = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Error in temp");

        let temp: f32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That is not a correct temperature type");
                continue;
            },
        };

        if option == 1 {
            fahrenheit_to_celsius(temp);
            break;
        } else if option == 2 {
            celsius_to_fahrenheit(temp);
            break;
        } else {
            println!("That is not an option!");
            continue;
        }
    }
}

fn fahrenheit_to_celsius(temp: f32) {
    let conversion: f32 = (temp - 32.0) * (0.56);
    println!("Your temp in Celsius is {conversion}");
}

fn celsius_to_fahrenheit(temp: f32) {
    let conversion: f32 = temp * (1.8) + 32.0;
    println!("Your temp in Fahrenheit is {conversion}");
}
