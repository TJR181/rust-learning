fn main() {
    println!("Hello, world!");
    another_function(5);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn another_function(x: i32) {
    println!("The value of x is {x}");
    let y = five();
    println!("Five is {y}");
    let x = 1;
    let x = plus_one(x);
    println!("1 + 1 = {x}");
}
