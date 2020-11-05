fn main() {
    // A string can be constructed using the String (class)
    let string_one: String = String::from("A string stored in the heap");
    println!("{}", string_one);

    // Also, a string can be stored in the stack using a slice instead
    let string_slice_one: &str = "Another string, stored on the stack.";
    println!("{}", string_slice_one);

    // Converting a slice to a normal String
    let string_from_slice1: String = String::from(string_slice_one);
    println!("{}", string_from_slice1);

    let string_from_slice2: String = "Hard coded string".to_string();
    println!("{}", string_from_slice2);

    // The other way around is much simpler and rust can easily translate a String to slice
    // Just prepend the & symbol to the string obj variable name. &string_one
    let string_from_obj: &str = &string_one;
    println!("{}", string_from_obj);

    // Combining strings
    let combined_string_literals = ["first", "second"].concat();
    // You can use format! to combine strings too
    let combined_with_format = format!("{} {}", "fourth", "third");
    println!("{}", combined_string_literals);
    println!("{}", combined_with_format);
}
