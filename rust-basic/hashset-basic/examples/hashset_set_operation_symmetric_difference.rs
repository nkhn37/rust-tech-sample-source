use std::collections::HashSet;

fn main() {
    let set_a: HashSet<_> = [1, 2, 3, 4].into_iter().collect();
    let set_b: HashSet<_> = [3, 4, 5, 6].into_iter().collect();

    // 排他的論理和（片方にだけ含まれる要素）
    let symmetric_difference: HashSet<_> = set_a.symmetric_difference(&set_b).copied().collect();

    println!("A: {set_a:?}");
    println!("B: {set_b:?}");
    println!("A △ B: {symmetric_difference:?}");
}
