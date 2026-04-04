fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // fold を使用して、イテレータの要素の合計を求める
    let sum = numbers.iter().fold(0, |acc, x| acc + x);

    // 結果を表示
    println!("sum: {}", sum);
}
