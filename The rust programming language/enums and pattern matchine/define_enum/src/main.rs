#[derive(Debug)]
#[allow(dead_code)]
enum IpAddress {
    V4(u8, u8, u8, u8),
    V6(String)
}

fn main() {
    let v4_addr = IpAddress::V4(127, 0, 0, 1);
    let v6_addr = IpAddress::V6(String::from("::1"));
    println!("{:?}", v4_addr);
    println!("{:?}", v6_addr);
}
