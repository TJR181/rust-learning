struct CustomSmartPointer {
    data: String
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data: {}", self.data);
    }
}

fn main() {
    let stuff_1 = CustomSmartPointer{data: String::from("My stuff")};
    let stuff_2 = CustomSmartPointer{data: String::from("Other stuff")};
    println!("CustomSmartPointer created");
}
