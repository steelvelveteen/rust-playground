use std::mem;

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    address: String,
}

fn main() {
    let mut numbers: Vec<i32> = vec![10, 20, 34, 45, 56];
    println!("{:?}", numbers);

    // Some vectors built in methods
    println!("Length: {}", numbers.len());
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));
    numbers.push(67); // need to make numbers mutable
    numbers.pop();

    for x in numbers.iter() {
        println!("{}", x);
    }

    println!("");
    // user iter_mut() if you want change the values at the same time you are iterating over the list
    for x in numbers.iter_mut() {
        println!("{}", *x + 1);
    }

    let p1 = Person {
        name: String::from("Joey"),
        age: 46,
        address: String::from("82 Jeffcott St"),
    };

    let p2 = Person {
        name: String::from("Mary Elizabeth Winstead"),
        age: 36,
        address: String::from("Somewhere in North Carolina"),
    };

    let p3 = Person {
        name: String::from("Amy Adams"),
        age: 46,
        address: String::from("Somehwere in California"),
    };

    let people: Vec<Person> = vec![p1, p2, p3];
    for p in people {
        println!("{:?}", p);
    }

    // Let's create a constructor for Person, impl method syntax
    impl Person {
        fn new(name: String, age: u32, address: String) -> Person {
            Person { name, age, address }
        }
    }

    let p4 = Person::new(
        String::from("Jennifer Lawrence"),
        34,
        String::from("California"),
    );
    println!("{:?}", p4);
}
