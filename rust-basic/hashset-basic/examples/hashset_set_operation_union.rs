use std::collections::HashSet;

fn main() {
    let set_a: HashSet<_> = [1, 2, 3].into_iter().collect();
    let set_b: HashSet<_> = [3, 4, 5].into_iter().collect();

    // 和集合（どちらかに含まれる要素）
    let union: HashSet<_> = set_a.union(&set_b).copied().collect();

    println!("A: {set_a:?}");
    println!("B: {set_b:?}");
    println!("A ∪ B: {union:?}");
}
