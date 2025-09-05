use std::{fs::File, io::ErrorKind};
#[allow(unused_variables)]
fn main() {
    let file = File::open("flag.txt");
    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("flag.txt") {
                Ok(file) => file,
                Err(error) => panic!("Failed to create file!")
            }
            other_error => panic!("Error when opening the file")
        }
    };

    // unwrap: can't set error message
    // expect: can set error message
    let file2 = File::open("flag").expect("can't open file!");
    
}
