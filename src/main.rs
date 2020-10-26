fn main() {
    // Function that takes parameters
    let x: i32 = 16;
    let y: i32 = 16;
    let result = function_with_params(x, y);
    println!("{}", result);
}

// In function signatures you MUST declare the type of each parameter
fn function_with_params(x: i32, y: i32) -> i32 {
    x * y
}
