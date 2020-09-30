// The print macro. Formatting
pub fn run() {
    println!("The print macro.");

    // Basic formatting
    println!("Number: {}", 1);

    // Positional arguments
    println!("{0} - {1} - {2}", "one", 2, true);
    println!("{2} {0} {3} {1}", "does", "matter", "order", "not");
    // Named arguments
    println!(
        "{name} likes to play {activity}",
        name = "John",
        activity = "teto"
    );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "kanpai"));
}
