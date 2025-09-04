fn main() {
    let mut i = 0;
    'outside_loop: loop {
        loop {
            i = i + 1;
            println!("The value is {i}");
            if i == 10 {
                break 'outside_loop;
            }
        }
    }
}
