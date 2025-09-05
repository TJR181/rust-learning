#[allow(dead_code)]
pub mod pub_pri {
    pub fn public_function() {
        println!("You should see this!");
    }

    fn private_function() {
        println!("You should not see this!")
    }
}