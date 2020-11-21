// Sorry, no match here. What I am exploring here is the use of the
// if let construct to avoid having to write all the boilerplate code of a match
// to exhaust all possiblities

use std::io;

fn main() {
    let some_u8_value = Some('N');
    if let Some('y') | Some('Y') = some_u8_value {
        println!("Yes");
    } else {
        println!("No");
    }

    println!("Enter first name: ");
    let mut first_name = String::new();
    io::stdin()
        .read_line(&mut first_name)
        .expect("Failed to enter name");
    println!("{}", first_name);
}
