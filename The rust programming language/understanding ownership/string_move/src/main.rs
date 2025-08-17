fn main() {
    let mut string_1 = String::from("Hello ");
    let mut string_2 = string_1.clone();
    string_2.push_str("world!");
    println!("string1: {}", string_1);
    println!("string2: {}", string_2);
}
