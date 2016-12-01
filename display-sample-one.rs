// 1.2.2 Display
use std::fmt;

struct Structure(i32);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() {
    let structure = Structure(4);

    println!("Display: {}", structure);
}