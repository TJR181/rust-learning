use std::fmt::Display;

pub struct Pair<T> {
    pub x: T,
    pub y: T
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest number is X = {}", self.x);
        }
        else {
            println!("The largest number is Y = {}", self.y);
        }
    }
}