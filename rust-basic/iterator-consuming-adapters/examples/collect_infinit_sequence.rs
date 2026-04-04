fn main() {
    // 無限の数値列を生成するイテレータ
    let infinite_numbers = 1..;

    // 無限の数値列から最初の 5 個を collect して Vec に集約
    let result_vec: Vec<_> = infinite_numbers.take(5).collect();

    // 結果を表示
    println!("{:?}", result_vec);
}
