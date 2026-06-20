use std::collections::HashSet;

fn main() {
    // 配列から HashSet を生成する
    let set: HashSet<_> = [1, 2, 3, 4].into_iter().collect();

    println!("生成された HashSet : {set:?}");
}
