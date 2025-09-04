struct StructDemo {
    name: String,
    email: String,
    age: u32,
    active: bool,
}

fn main() {
    let test_struct = StructDemo {
        name: String::from("r1ver"),
        email: String::from("tianjingren@tjr181.com"),
        age: 22,
        active: true,
    };

    println!(
        "name: {}\nemail: {}\nage: {}\nactive: {}\n\n",
        test_struct.name, test_struct.email, test_struct.age, test_struct.active
    );

    // 更新语法
    let test_struct_2 = StructDemo {
        name: String::from("tjr"),
        ..test_struct
    };

    println!(
        "name: {}\nemail: {}\nage: {}\nactive: {}",
        test_struct_2.name, test_struct_2.email, test_struct_2.age, test_struct_2.active
    );
}
