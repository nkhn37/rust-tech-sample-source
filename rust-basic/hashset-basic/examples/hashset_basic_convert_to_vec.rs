use std::collections::HashSet;

fn main() {
    let set: HashSet<_> = [3, 1, 4, 2].into_iter().collect();

    // HashSet から Vec に変換する
    let mut numbers: Vec<_> = set.into_iter().collect();

    // HashSet は順序を持たないため、見やすいように並べ替える
    numbers.sort();

    println!("変換後の Vec : {numbers:?}");
}
