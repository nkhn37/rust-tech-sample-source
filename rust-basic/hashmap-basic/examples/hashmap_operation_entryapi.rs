use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Alice"), 80);

    println!("初期状態 : {:?}", scores);

    // Alice はすでに存在するので更新されない
    scores.entry(String::from("Alice")).or_insert(0);

    // Bob は存在しないので追加される
    scores.entry(String::from("Bob")).or_insert(100);

    println!("or_insert 後 : {:?}", scores);
}
