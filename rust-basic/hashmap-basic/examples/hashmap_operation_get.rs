use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    // Alice のスコアを追加
    scores.insert(String::from("Alice"), 80);

    // Alice のスコアを取得
    let alice_score = scores.get("Alice");
    match alice_score {
        Some(score) => println!("Alice のスコア : {}", score),
        None => println!("Alice のスコアが見つかりませんでした。"),
    }

    // Bob のスコアを取得（存在しないキー）
    let bob_score = scores.get("Bob");
    match bob_score {
        Some(score) => println!("Bob のスコア : {}", score),
        None => println!("Bob のスコアが見つかりませんでした。"),
    }
}
