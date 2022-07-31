use std::io;

fn main() {
    const FAHRENHEIT: u8 = 1;
    const CELSIUS: u8 = 2;

    println!("From:");
    println!("1 - Fahrenheit");
    println!("2 - Celsius");
    println!("Option: ");

    let from = get_option();

    match from {
        FAHRENHEIT => println!("{}°C", to_celsius()),
        CELSIUS => println!("{}°F", to_fahrenheit()),
        _ => ()
    };
}

fn get_option() -> u8 {
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("wtf");

        let option: u8 = match input.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("You should type a number");
                continue;
            }
        };

        if option == 1 || option == 2 {
            return option
        }

        println!("Invalid option...");
    }
}

fn get_degrees() -> f32 {
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("wtf");

        match input.trim().parse() {
            Ok(degrees) => break degrees,
            Err(_) => println!("You should type a number"),
        }
    }
}

fn to_celsius() -> f32 {
    println!("Degrees in fahrenheit: ");

    let degrees = get_degrees();

    (degrees - 32 as f32) * 0.5556
}

fn to_fahrenheit() -> f32 {
    println!("Degrees in celsius: ");

    let degrees = get_degrees();

    (degrees * 1.8) + 32 as f32
}
