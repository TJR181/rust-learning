#[derive(Debug)]
#[allow(dead_code)]
struct Number<T> {
    x: T,
    y: T
}

fn main() {
    let struct1 = Number {x: 5, y: 6};
    let struct2 = Number {x: 5.1, y: 6.234};
    println!("{:?}", struct1);
    println!("{:?}", struct2);
}
