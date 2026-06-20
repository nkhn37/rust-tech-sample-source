use std::collections::HashSet;

fn main() {
    let numbers = vec![1, 2, 2, 3, 3, 4];

    // Vec から HashSet を生成する（重複は自動で除かれる）
    let set: HashSet<_> = numbers.into_iter().collect();

    println!("元の Vec : [1, 2, 2, 3, 3, 4]");
    println!("生成された HashSet : {set:?}");
}
