use std::collections::HashSet;

fn main() {
    let mut names = HashSet::new();

    let len = names.len();
    println!("初期状態の要素数 : {len}");

    names.insert(String::from("Alice"));
    names.insert(String::from("Bob"));

    let len = names.len();
    println!("追加後の要素数 : {len}");

    names.remove("Alice");

    let len = names.len();
    println!("削除後の要素数 : {len}");
}
