use std::collections::HashSet;

fn main() {
    let mut names = HashSet::new();

    names.insert(String::from("Alice"));
    names.insert(String::from("Bob"));
    names.insert(String::from("Charlie"));

    // 所有権を移動してループする
    for name in names {
        println!("名前 : {name}");
    }

    // 所有権が移動したため、ここで names は使えない
    // println!("{:?}", names); // コンパイルエラー
}
