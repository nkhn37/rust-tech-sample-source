use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    println!("初期状態 : {:?}", scores);

    // スコアを追加する
    scores.insert(String::from("Alice"), 80);
    scores.insert(String::from("Bob"), 100);

    println!("追加後 : {:?}", scores);

    // Alice のスコアを更新する（上書き）
    scores.insert(String::from("Alice"), 95);

    println!("更新後 : {:?}", scores);
}
