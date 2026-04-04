use std::collections::HashSet;

fn main() {
    let numbers = vec![1, 2, 3, 1, 2, 5];

    // HashSet を使用して、重複を排除したコレクションに集約
    let result_set: HashSet<_> = numbers.iter().map(|x| x * 2).collect();

    // 結果を表示
    println!("{:?}", result_set);
}
