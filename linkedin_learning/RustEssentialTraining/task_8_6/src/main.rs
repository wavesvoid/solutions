use std::io;
use rand::prelude::*;


fn main() {
    let mut input_buffer = String::with_capacity(10);
    let secret_number = thread_rng().gen_range(1..=100);

    loop {
        println!("Please enter your guess:");
        io::stdin().read_line(&mut input_buffer);
        
        let guess: i32 = input_buffer.trim().parse().map_err(|e| {
            println!("Cannot correctly read the number. Wrong input!");
            e
        }).unwrap();
        match secret_number.cmp(&guess) {
            std::cmp::Ordering::Less => println!("Too high!"),
            std::cmp::Ordering::Greater => println!("Too low!"),
            _ => {
                println!("Success !! {}", guess);
                break;
            }
        }
        input_buffer.clear();
    }
}
