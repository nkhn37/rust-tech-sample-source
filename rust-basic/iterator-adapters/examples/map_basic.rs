fn main() {
    // 元データのベクタを作成
    let numbers = vec![1, 2, 3];

    // map を使用し、各要素を2倍にしたイテレータを作成
    // (この時点ではまだ計算されない)
    let doubled_iter = numbers.iter().map(|x| x * 2);

    // イテレータをベクタに収集 (ここで初めて計算が行われる)
    let result: Vec<_> = doubled_iter.collect();

    // 結果を表示
    println!("{:?}", result);
}
