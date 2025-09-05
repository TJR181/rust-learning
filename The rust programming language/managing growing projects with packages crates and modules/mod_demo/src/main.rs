
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            super::write_something();
        }
    }
    fn write_something() {
        println!("666");
    }
}

fn main() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
}
