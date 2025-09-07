fn main() {
    let string1 = String::from("R1ver");
    let string2 = "Rust123";
    println!("The longest string is: {}", longest(&string1, &string2));
}

fn longest<'a>(string1: &'a str, string2: &'a str) -> &'a str {
    if string1.len() > string2.len() {
        string1
    }
    else {
        string2
    }
}