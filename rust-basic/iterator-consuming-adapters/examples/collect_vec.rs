fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // map を適用したイテレータを collect して新しい Vec に集約
    let result_vec: Vec<_> = numbers.iter().map(|x| x * 2).collect();

    // 結果を表示
    println!("{:?}", result_vec);
}
