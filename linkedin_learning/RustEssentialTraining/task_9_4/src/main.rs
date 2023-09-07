//! Check if a file contains a specific name

use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};


fn main() {
    let mut args = env::args().skip(1).take(2);

    if args.len() != 2 {
        println!("Provide exactly two arguments: {filename} {name}");
        return;
    }
    
    let (filename, search_name) = (args.next().unwrap(), args.next().unwrap());
    let file = File::open(&filename)
        .expect(&format!("Cannot open {}", &filename));
    let mut linesbuf = BufReader::new(file).lines();

    while let Some(Ok(line)) = linesbuf.next() {
        if line.eq(&search_name) {
            println!(
                "This file contains given item ({}) in the list", search_name);
            return;
        }
    }

    println!("This item ({}) not found in list", search_name);
}
