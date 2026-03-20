fn main() {
    // +演算子で文字列を結合する
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    
    // s1 の所有権はムーブするので注意、s2 は参照を渡す必要がある
    let s3 = s1 + &s2;
    println!("{}", s3);

    // s1 の所有権は移動するので以下はコンパイルエラー
    // println!("{}", s1);

    // format マクロで結合する（ムーブなしで結合可能）
    let s1 = String::from("Hello");
    let s2 = String::from("World");

    // format で結合
    let s3 = format!("{}{}", s1, s2);
    println!("{}", s3);
    // ムーブしないので元の変数も使用できる。
    println!("{}", s1);
    println!("{}", s2);
}