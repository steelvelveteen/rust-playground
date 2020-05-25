mod user;

fn main() {
    println!("Working with structs in Rust");
    // Create a simple struct


    #[derive(Debug)]
    struct Employee {
        firstname: String,
        lastname: String,
        age: i8,
        position: String
    };

    #[derive(Debug)]
    struct Person {
        name: &'static str,
        address: &'static str,
        city: &'static str
    };

    let person = Person {
        name: "Joey",
        address: "82 Jeffcott St.",
        city: "Adelaide"
    };

    println!("{:?}", person);

    let employee = Employee {
        firstname: String::from("Sonoya"),
        lastname: String::from("Mizuno"),
        age: 32,
        position: String::from("Natural born dancer"),
    };

    println!("{:?}", employee);

    let user = user::build_user("sonoya@gmail.com", "Sonoya");

    println!("{:?}", user);
}


