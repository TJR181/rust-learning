fn main() {
    let s = String::from("Hello world");
    let word_index = first_world(&s);
    print!("{word_index}");
}

fn first_world(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s
}

// fn main() {
//     let s = String::from("Hello world");
//     let hello = &s[..5];
//     let world = &s[6..];
//     let whole = &s[..];

//     println!("{}, {}", hello, world);
//     println!("whole: {}", whole);
// }