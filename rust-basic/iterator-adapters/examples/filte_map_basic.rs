fn main() {
    let values = vec!["10", "abc", "20", "def", "30"];

    // filter_map を使用して、文字列を整数に変換し、成功したものだけを収集する
    let numbers: Vec<_> = values
        .iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    // 結果を表示
    println!("{:?}", numbers);
}
