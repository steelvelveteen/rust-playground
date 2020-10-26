
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
}