use std::collections::HashSet;

fn main() {
    let mut names = HashSet::new();

    names.insert(String::from("Alice"));
    names.insert(String::from("Bob"));

    // 要素を取得する
    let alice = names.get("Alice");
    match alice {
        Some(name) => println!("取得できた要素 : {name}"),
        None => println!("要素が見つかりませんでした。"),
    }

    let charlie = names.get("Charlie");
    match charlie {
        Some(name) => println!("取得できた要素 : {name}"),
        None => println!("Charlie は存在しません。"),
    }
}
