use std::collections::HashSet;

fn main() {
    let mut names = HashSet::new();

    println!("追加前 : {names:?}");

    // 要素を追加する
    names.insert(String::from("Alice"));
    names.insert(String::from("Bob"));

    println!("追加後 : {names:?}");

    // 同じ値を追加しても重複しない
    names.insert(String::from("Alice"));

    println!("同じ値を再追加した後 : {names:?}");
}
