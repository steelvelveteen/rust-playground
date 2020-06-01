// Usage of generics in function definitions

fn main() {
    let list_of_numbers = [34, 55, 1, 9, 0, 23, 46, 66];
    let big_as_fuck: i32 = largest_i32(&list_of_numbers);
    println!("The largest number found was: {}", big_as_fuck);

    let list_of_chars = ['y', 'm', 'a', 'b'];
    let big_char: char = largest_char(&list_of_chars);
    println!("The largest char was: {}", big_char);
}

// We place the generics in the signature of the function
// where we would usually specify the data types of the parameters
// and return value

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
