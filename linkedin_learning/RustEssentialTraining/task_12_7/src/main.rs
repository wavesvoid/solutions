struct Satellite {
    name: String,
    velocity: f64 // miles per second
}

/* YOUR CODE GOES HERE */
use std::fmt::{Display, Error, Formatter};
impl Display for Satellite {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "Satellite {{\n\tname: {},\n\tvelocity: {}\n}}", 
            self.name,
            self.velocity)
    }

}


fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72
    };
    println!("hubble is {}", hubble);
}
