fn main() {
    //let vector: Vec<u32> = Vec::new();
    let vector = vec![1, 2, 3, 4, 5];
    let third = &vector[2];
    println!("The third element is {}", third);

    match vector.get(2) {
        Some(num) => println!("The third element is {}", num),
        None => println!("There is no third element")
    }
    
}
