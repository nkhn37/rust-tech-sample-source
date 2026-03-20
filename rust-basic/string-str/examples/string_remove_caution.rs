fn main() {
    let mut s1 = String::from("Rust、こんにちは。");
    
    // 以下はコンパイルエラーとなる
    // s1.remove(5);

    // 適切な位置であれば削除ができる。
    s1.remove(4);
    println!("{}", s1);

    // 文字数でバイト位置を検索してから削除する
    let mut s2 = String::from("Rust、こんにちは。");
    if let Some((idx, _)) = s2.char_indices().nth(10) {
        s2.remove(idx);
    }
    println!("{}", s2);
}
