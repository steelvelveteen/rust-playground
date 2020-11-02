#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Rectangle {
    width: u32,
    height: u32
}
// For now here I am using String objects
// slices require lifetime specifier
fn main() {
    let user = User {
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
        active: true,
    };

    println!("{:#?}", user);

    let rect = Rectangle {
        width: 30,
        height: 45
    };

    println!("The area of the rectangle is {} square units.", area(&rect));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
