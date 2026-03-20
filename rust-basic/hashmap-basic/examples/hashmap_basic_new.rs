use std::collections::HashMap;

fn main() {
    // HashMap を生成する
    let mut scores = HashMap::new();

    // キー、値を追加する
    scores.insert(String::from("Alice"), 80);
    scores.insert(String::from("Bob"), 100);

    println!("{:?}", scores);
}
