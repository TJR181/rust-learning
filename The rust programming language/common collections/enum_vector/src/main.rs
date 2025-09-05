#[derive(Debug)]
#[allow(dead_code)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}

fn main() {
    let enum_vector = [
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(3.1415926),
        SpreadsheetCell::Text(String::from("Hello"))
    ];
    println!("{:?}", &enum_vector[0]);
    println!("{:?}", &enum_vector[1]);
    println!("{:?}", &enum_vector[2]);
}
