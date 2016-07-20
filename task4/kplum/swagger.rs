use std::fmt::Display;
use std::fmt::Result;
use std::fmt::Formatter;

struct Swagger<T> {
    x: T,
}

impl<T: Display> Display for Swagger<T> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "#swag {} #yolo", self.x)
    }
}

fn main() {
    let blub = Swagger { x: 4711 };
    println!("{}", blub);
}
