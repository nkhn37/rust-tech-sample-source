use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Alice"), 80);
    scores.insert(String::from("Bob"), 100);
    scores.insert(String::from("Charlie"), 70);

    println!("削除前 : {:?}", scores);

    // キーを指定して削除する
    scores.remove("Alice");

    println!("Alice 削除後 : {:?}", scores);

    // 存在しないキーを削除しても何も起きない
    scores.remove("Dave");

    println!("Dave 削除後（変化なし） : {:?}", scores);
}
