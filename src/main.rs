#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 45,
    };

    // Function form
    println!("The area of the rectangle is {} square units.", area(&rect));

    // Method form using implement
    println!("The area of the rectangle is {} square units", rect.area());
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
