use std::collections::HashMap;

fn main() {
    let mut test_map: HashMap<String, i32> = HashMap::new();
    test_map.insert(String::from("TJR"), 181);
    test_map.insert(String::from("r1ver"), 666);

    // another way to create HashMap
    let teams = vec![String::from("team a"), String::from("team b")];
    let intial_scores = vec![80, 85];

    let scores: HashMap<_, _> = teams.iter().zip(intial_scores.iter()).collect();
    
    match scores.get(&String::from("team b")) {
        Some(num) => println!("The score is {}", num),
        None => println!("Can't find such team")
    }

    for (k, v) in scores {
        println!("Key: {}", k);
        println!("Value: {}", v);
    }
    // scores.insert: update the value, don't care if there are already have a same key
    // scores.entry ... or_insert: update the value when there is no same key

    let test_string = String::from("world hello hello hello world haha");
    let mut string_times = HashMap::new();

    for i in test_string.split_whitespace() {
        let count = string_times.entry(i).or_insert(0);
        *count+=1;
    }
    println!("{:#?}", string_times);

}
