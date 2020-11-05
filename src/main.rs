#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    let addr = IpAddr::V4("v4 ip address".to_string());
    let addr2 = IpAddr::V6(String::from("V6 ip address"));

    println!("Ip Address type V4: {:#?}", addr);
    println!("Ip Address type V6: {:#?}", addr2);

    // Found a way to print the string value contained in an 
    // enum using Option<T>
    let addr = Some(IpAddr::V4(String::from("Some string")));
    match addr {
        Some(IpAddr::V4(val)) | Some(IpAddr::V6(val)) => println!("{}", val),
        // Some(IpAddr::V6(val)) => println!("{}", val),
        None => println!("No"),
    }
}
