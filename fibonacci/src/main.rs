use std::io;

fn main() {
    println!("Give me the nth fibonacci number: ");

    let position = get_position();

    println!("The {} position value is: {}", position, get_fibonacci_position_value(position));
}

fn get_position() -> u32 {
    loop {
        let mut position = String::new();

        io::stdin()
            .read_line(&mut position)
            .expect("Something really weird happened");

        match position.trim().parse() {
            Ok(value) => break value,
            Err(_)        => print!("You should type a valid number")
        }
    }
}

fn get_fibonacci_position_value(position: u32) -> u32 {
    if position <= 1 {
        return position
    }

    get_fibonacci_position_value(position - 1) + get_fibonacci_position_value(position - 2)
}
