use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Alice"), 80);
    scores.insert(String::from("Bob"), 100);
    scores.insert(String::from("Charlie"), 70);

    // 所有権を移動せず参照としてループする
    for (name, score) in &scores {
        println!("{name} のスコア : {score}");
    }

    // ループ後も scores を使用できる
    println!("ループ後のエントリ数 : {}", scores.len());
}
