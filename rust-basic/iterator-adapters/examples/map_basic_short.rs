fn main() {
    // 元データのベクタを作成
    let numbers = vec![1, 2, 3];

    // map を使用し、各要素を2倍にした結果を取得する
    let result: Vec<_> = numbers.iter().map(|x| x * 2).collect();

    // 結果を表示
    println!("{:?}", result);
}
