fn append_message(s: &mut String) {
    // 可変参照なので変更可能
    s.push_str("!!!");
}

fn main() {
    let mut s1 = String::from("Hello World");

    // s1 の内容を可変参照で関数に渡す
    append_message(&mut s1);

    // 関数側で変更された結果が表示される
    println!("{}", s1);
}
