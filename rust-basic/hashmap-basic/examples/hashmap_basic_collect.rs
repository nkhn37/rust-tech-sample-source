use std::collections::HashMap;

fn main() {
    let keys = vec![String::from("Alice"), String::from("Bob")];
    let values = vec![80, 10];

    let scores: HashMap<String, i32> = keys.into_iter().zip(values.into_iter()).collect();

    println!("{:?}", scores);
}
