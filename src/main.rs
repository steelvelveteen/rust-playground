// Understanding stack vs heap in Rust
// All data on the stack must a have a known, fixed size
// The heap allocates a certain amount of space by request

// Ownership rules
// 1 Each value in Rust has variable that's called its owner
// 2 There can only be one owner at a time
// 3 When owner goes out of scope value is dropped

// This branch is actually about the usage of references

fn main() {
    let mut name_string = String::from("Joey Vico");
    println!("Original name: {}", name_string);

    name_string = change_name(&mut name_string).to_string();
    println!("Changed name: {}", name_string);
}

fn change_name(name: &mut String) -> &String {
    name.push_str(" Lopez");
    name
}
