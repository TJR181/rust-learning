#[allow(dead_code)]
pub struct Guess {
    value: i32
}

impl Guess {
    pub fn new(value: i32) {
        if value < 1 || value > 100 {
            panic!("Guess number must between 1 and 100, current number is {}", value);
        }
        Guess { value };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Guess number must between 1 and 100")]
    fn greater_than_100() {
        Guess::new(101);
    }
}