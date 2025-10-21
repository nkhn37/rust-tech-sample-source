fn main() {
    let info: (i32, f64, bool) = (100, 0.123, true); // 型の順番と個数が重要
    let (i_val, f_val, flag) = info; // タプルに対するパターンマッチによる値の束縛 (デストラクチャリング)

    // タプルへのアクセス
    println!("infoの値: {:?}", info);
    println!("1つ目の要素: {}", info.0);
    println!("2つ目の要素: {}", info.1);
    println!("3つ目の要素: {}", info.2);
    // println!("エラーになる: {}", info.3);

    // デストラクチャリングの結果を表示
    println!("i_valの値: {}", i_val);
    println!("f_valの値: {}", f_val);
    println!("flagの値: {}", flag);
}
