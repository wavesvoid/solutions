use std::io::{stdin, Error};


fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Cannot read the string!");
    
    println!("Hello, World.\n{input}");
}
