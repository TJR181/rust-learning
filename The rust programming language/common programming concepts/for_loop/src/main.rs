fn main() {
    let a = [1, 2, 3, 4, 5];
    for element in a {
        println!("{element}");
    }

    for number in (1..5).rev() {
        println!("{number}");
    }
}
