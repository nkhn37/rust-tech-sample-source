fn main() {
    let numbers = [3, 2, 5, 1, 4];

    // 最大値と最小値を求める
    let max = numbers.iter().max();
    let min = numbers.iter().min();

    // 結果を表示
    println!("max: {:?}", max);
    println!("min: {:?}", min);
}
