use std::collections::HashSet;

fn main() {
    let mut names = HashSet::new();

    names.insert(String::from("Alice"));
    names.insert(String::from("Bob"));
    names.insert(String::from("Charlie"));

    println!("削除前 : {names:?}");

    // 要素を削除する
    names.remove("Alice");

    println!("Alice 削除後 : {names:?}");

    // 存在しない要素を削除しても変化しない
    names.remove("Dave");

    println!("Dave 削除後（変化なし） : {names:?}");
}
