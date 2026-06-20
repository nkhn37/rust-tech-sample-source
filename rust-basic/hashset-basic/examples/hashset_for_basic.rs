use std::collections::HashSet;

fn main() {
    let mut names = HashSet::new();

    names.insert(String::from("Alice"));
    names.insert(String::from("Bob"));
    names.insert(String::from("Charlie"));

    // 所有権を移動せず、参照としてループする
    for name in &names {
        println!("名前 : {name}");
    }

    // ループ後も names を使える
    let len = names.len();
    println!("ループ後の要素数 : {len}");
}
