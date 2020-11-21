fn main() {
    let x = 5;

    // Simple match, just like the switch in other programming
    // languages
    match x {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        4 => println!("Four"),
        5 => println!("Five"),
        _ => println!("Not found")
    }

    // A more complex one
    let n = 15;
    match n {
        1 => println!("This is one"),
        2 | 3 | 5 | 7 | 11 => println!("Prime number"),
        // Below is the new syntax apparently
        13..=19 => println!("A teen"),
        _ => println!("Ain't that special of a number")
    }

    // A much more complex match pattern
    // with tuples
    let pair = (0, -2);
    match pair {
        (0, y) => println!("y: {}", y),
        (x, 0) => println!("x: {}", x),
        _ => println!("No match!")
    }

    let n = 1;
    let answer = match n {
        1 => 1,
        2 | 3 | 5 | 7 | 11 => n,
        // Line below is the new syntax apparently for inclusive ranges
        13..=19 => n,
        _ => 0
    };

    println!("Answer: {}", answer);

    // binding
    let p = 5;
    match p {
        n @ 1..=12 => println!("n: {}", n),
        n @ 13..=19 => println!("n: {}", n),
        _ => println!("No match")
    }
}
