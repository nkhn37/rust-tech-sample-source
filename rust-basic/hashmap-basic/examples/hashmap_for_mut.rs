use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Alice"), 80);
    scores.insert(String::from("Bob"), 100);
    scores.insert(String::from("Charlie"), 70);

    println!("更新前 : {:?}", scores);

    // 可変参照でループし、全員のスコアに 10 を加算する
    for (_name, score) in &mut scores {
        *score += 10;
    }

    println!("更新後 : {:?}", scores);
}
