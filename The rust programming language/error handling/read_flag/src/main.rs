use std::io::{self, Read};
use std::fs::File;
use std::error::Error;

fn read_flag(flag_name: &String) -> Result<String, io::Error> {
    // open flag
    let mut file = File::open(flag_name)?;
    let mut flag = String::new();
    file.read_to_string(& mut flag)?;
    Ok(flag)
}

fn main() -> Result<(), Box<dyn Error>> {
    let flag_addr = String::from("flag.txt");
    let flag = read_flag(&flag_addr).unwrap();
    println!("{}", flag);
    let mut same_flag = String::new();
    File::open(flag_addr)?.read_to_string(&mut same_flag)?;
    Ok(())
}
