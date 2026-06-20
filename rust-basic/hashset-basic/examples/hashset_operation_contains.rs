use std::collections::HashSet;

fn main() {
    let mut names = HashSet::new();

    names.insert(String::from("Alice"));
    names.insert(String::from("Bob"));

    // 要素の存在を確認する
    let contains_alice = names.contains("Alice");
    let contains_charlie = names.contains("Charlie");
    println!("Alice は存在するか : {contains_alice}");
    println!("Charlie は存在するか : {contains_charlie}");
}
