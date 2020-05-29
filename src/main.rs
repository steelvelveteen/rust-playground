// Traits are like interfaces
// Suppose you want to implement the fmt::Display trait
// You use impl fmt::Display for {object} and the compiler will
// warn you use that you're missing the fmt() function
use std::fmt;

#[derive(Debug)]
struct Object {
    width: u32,
    height: u32,
}

impl Object {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

// Implementing a trait from the std library
impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} units width - {} units height",
            &self.width, &self.height,
        )
    }
}

// Create your own trait
// Rem: a trait is like an interface that contains a function definition
pub trait SomeTrait {
    fn print_object(&self);
}

impl SomeTrait for Object {
    fn print_object(&self) {
        println!("{:?}", &self);
    }
}

fn main() {
    let struct1 = Object::new(8, 12);
    // The println macro will not know how to print out the struct
    // You may use the {:?} but let's use our own implementation
    println!("{}", struct1);
    struct1.print_object();
}
