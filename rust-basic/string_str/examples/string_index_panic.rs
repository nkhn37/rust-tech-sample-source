fn main() {
    // バイト位置を指定する場合は、マルチバイト文字の時に注意
    // ===== insert の場合
    let mut s1 = String::from("Rust、こんにちは。");

    // 以下は panic となる
    // s1.insert(5, '!');

    // 文字数でバイト位置を検索してからであれば安全
    if let Some((idx, _)) = s1.char_indices().nth(10) {
        s1.insert(idx, '!');
    }
    println!("{}", s1);

    // ===== remove の場合
    let mut s2 = String::from("Rust、こんにちは。");

    // 以下は panic となる
    // s1.remove(5);

    // 文字数でバイト位置を検索してからであれば安全
    if let Some((idx, _)) = s2.char_indices().nth(4) {
        s2.remove(idx);
    }
    println!("{}", s2);

    // ===== truncate の場合
    let mut s3 = String::from("Rust、こんにちは。");

    // 以下は panic となる
    // s3.truncate(5);

    // 文字数でバイト位置を検索してからであれば安全
    if let Some((idx, _)) = s3.char_indices().nth(4) {
        s3.truncate(idx);
    }
    println!("{}", s3);
}
