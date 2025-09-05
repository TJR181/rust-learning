fn main() {
    let string_1 = String::from("Hello ");
    let string_2 = String::from("world");
    // let string_3 = string_1 + &string_2;
    let string_3 = format!("{}{}",string_1, string_2);
    println!("{}", string_3);
}
