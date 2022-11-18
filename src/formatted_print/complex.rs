use std::fmt::{Display};

#[derive(Debug)]
struct Complex {
    real: f64,
    img: f64,
}

impl Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} + {}i", self.real, self.img)
    }
}

pub fn main() {
    let cnum = Complex { real: 2.0, img: 3.14 };
    println!("complex number 1 is: {}", cnum);
    println!("Debug Format of complex number 1 is: {:?}", cnum);
}