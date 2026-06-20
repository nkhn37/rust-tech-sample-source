use std::collections::HashSet;

fn main() {
    // HashSet を生成する
    let mut set = HashSet::new();

    // 値を追加する
    set.insert(String::from("Alice"));
    set.insert(String::from("Bob"));
    set.insert(String::from("Charlie"));

    println!("{set:?}");
}
