#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn calculate_area(&self) -> u32 {
        self.width * self.length
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            length: size,
            width:size
        }
    }
}

fn main() {
    let test_rectangle = Rectangle {
        width: 11,
        length: 12,
    };
    let test_area = test_rectangle.calculate_area();
    println!("{:#?}\nresult: {}", test_rectangle, test_area);
    println!("\n\n{:?}", Rectangle::square(5));
}
