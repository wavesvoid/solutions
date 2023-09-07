use std::io;
use rand::prelude::*;


fn main() {
    let mut input_buffer = String::with_capacity(10);
    let secret_number = thread_rng().gen_range(1..=100);
    let mut guess: i32;

    loop {
        println!("Please enter your guess:");
        io::stdin().read_line(&mut input_buffer);
        
        match input_buffer.trim().parse() {
            Err(_) => {
                println!("Cannot correctly read the number. Wrong input!");
            }
            Ok(guess) => match secret_number.cmp(&guess) {
                std::cmp::Ordering::Less => println!("Too high!"),
                std::cmp::Ordering::Greater => println!("Too low!"),
                _ => {
                    println!("Success !! {}", guess);
                    break;
                }
            }
        }
        input_buffer.clear();
    }
}
