fn main() {
    let words = vec!["Hello", " ", "Rust", "!"];

    // fold を使用して、イテレータの要素を連結する
    let concatenated = words.iter().fold(String::new(), |acc, word| acc + word);

    // 結果を表示
    println!("{}", concatenated);
}
