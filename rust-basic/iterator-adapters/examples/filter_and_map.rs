fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // filter と map を組み合わせて、偶数の2倍を計算する
    let even_doubles: Vec<_> = numbers
        .iter()
        .filter(|x| **x % 2 == 0)
        .map(|x| x * 2)
        .collect();

    // 結果を表示
    println!("{:?}", even_doubles);
}
