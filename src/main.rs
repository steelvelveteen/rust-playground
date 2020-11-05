fn main() {
    let number = 3;

    // Nothing too glorious only that there are no braces
    // around the condition in the if expression
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Fun part here
    let condition = true;
    let number = if condition { 5 } else { 6 };

    // Because if is an expression it can be used on the right
    // side of a let statement
    println!("The value of number is: {}", number);

    // Repetition with loops
    // and to the right of a let statement !!
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 5 {
            // The value counter * 2 is what is
            // returned after the break expression occurs
            break counter * 2;
        }
    };
    println!("The final result of counter is {}", result);

    // Simple while loop. Nothing new in this
    let mut subcounter = 0;
    while subcounter != 5 {
        println!("Subcounter: {}", subcounter + 1);
        subcounter += 1;
    }

    // iterating for loops
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("Element in a: {}", element);
    }
}
