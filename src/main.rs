#[derive(Debug)]
enum Sex {
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
    sex: Sex,
    address: Address,
}

fn main() {
    let user_details = UserDetails {
        first_name: String::from("Sonoya"),
        last_name: String::from("Mizuno"),
        email: String::from("sonoya@gmail.com"),
        sex: Sex::Female,
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
