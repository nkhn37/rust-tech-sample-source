fn main() {
    let words = vec!["a", "b", "c", "d"];

    // fold を使用して、前から順に要素を結合する
    let forward = words.iter().fold(String::new(), |acc, word| acc + word);

    // rfold を使用して、逆順に要素を結合する
    let backward = words.iter().rfold(String::new(), |acc, word| acc + word);

    // 結果を表示
    println!("fold: {}", forward);
    println!("rfold: {}", backward);
}
