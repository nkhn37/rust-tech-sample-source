fn main() {
    let numbers = vec![1, 2, 3];

    let result: Vec<_> = numbers.iter().flat_map(|x| vec![*x, *x * 10]).collect();

    // 結果を表示
    println!("{:?}", result);
}
