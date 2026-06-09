use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Alice"), 80);
    scores.insert(String::from("Bob"), 100);
    scores.insert(String::from("Charlie"), 70);

    // 所有権を移動させてループする
    for (name, score) in scores {
        println!("{name} のスコア : {score}");
    }

    // 所有権が移動したため、ループ後は scores を使用できない
    // println!("{:?}", scores); // コンパイルエラー
}
