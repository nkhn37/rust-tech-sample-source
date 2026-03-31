fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // filter を使用し、偶数のみを抽出する
    let even_numbers: Vec<_> = numbers.iter().filter(|x| **x % 2 == 0).collect();

    // 結果を表示
    println!("{:?}", even_numbers);
}
