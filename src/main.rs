mod user;
use user::User;

#[derive(Debug)]
struct Person {
    name: &'static str,
    address: &'static str,
    city: &'static str
}

impl Person {
    fn new(name: &'static str, address: &'static str, city: &'static str) -> Person {
       Person {
            name,
            address,
            city,
        }
    }
}
fn main() {
    println!("Working with structs in Rust");

    let person = Person::new("Joey Vico", "joeyvico@gmail.com", "Adelaide");
    println!("{:?}", person);

    // Creating another user with using the builder
    // You will have to make each field public as well
    // Rust will consider everything private unless specified otherwise
    let user2 = User {
        email: "joeyvico@gmail.com",
        username: "joeyvico",
    };

    println!("{:?}", user2);

    // Using the method constructor new to create a new user
    let user3 = User::new("dianelane@gmail.com", "Diane");
    // println!("{:?}", user3);
    // Use the method to print out user's data instead
    User::display_user_data(&user3);
}

