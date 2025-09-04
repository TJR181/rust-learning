#[derive(Debug)]
struct Rectangular {
    length: u32,
    width: u32
}

fn main() {
    let test_rectangular = Rectangular{
        width: 11,
        length: 12
    };
    let test_area = calculate_area(&test_rectangular);
    println!("{:#?}\nresult: {}", test_rectangular, test_area);
}

fn calculate_area(rectangular: &Rectangular) -> u32 {
    rectangular.width * rectangular.length
}
