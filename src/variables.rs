pub fn run() {
    println!("Variables");
    let name = "Scarlett";
    let mut age = 37;
    println!("Her name is {} and she is {}", name, age);
    age = 38;
    println!("Her name is {} and she is {}", name, age);

    // Assigning multiple variables at once
    let (my_name, my_age) = ("Joey", 46);
    println!("{}, {}", my_name, my_age);
}
