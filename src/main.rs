#[derive(Debug)]
enum Genre {
    _Male,
    Female,
    _Unknown,
}

#[derive(Debug)]
struct Address {
    street: String,
    number: u32,
    code: u32,
}

#[derive(Debug)]
struct UserDetails {
    first_name: String,
    last_name: String,
    email: String,
    sex: Genre,
    address: Address,
}

fn main() {
    let user_details = UserDetails {
        first_name: String::from("Sonoya"),
        last_name: String::from("Mizuno"),
        email: String::from("sonoya@gmail.com"),
        sex: Genre::Female,
        address: Address {
            street: String::from("Jeffcott St."),
            number: 82,
            code: 5006,
        },
    };

    display_user_details(user_details);
}

fn display_user_details(user: UserDetails) {
    println!("User: {:#?}", user);
}
