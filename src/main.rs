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
}
