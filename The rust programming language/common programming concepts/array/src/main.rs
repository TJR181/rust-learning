use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];
    //let b = [3; 5];
    //let first = a[0];
    //let second = a[1];
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Failed to convert to u32!");

    let element = a[index];
    println!("The value of the element at index {index} is: {element}");

}
