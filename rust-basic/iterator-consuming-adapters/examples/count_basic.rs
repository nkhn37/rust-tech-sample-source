fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // count() メソッドを使用して、イテレータの要素数を数える
    let count = numbers.iter().count();

    // 結果を表示
    println!("{}", count);
}
