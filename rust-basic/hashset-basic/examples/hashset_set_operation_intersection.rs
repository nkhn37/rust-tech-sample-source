use std::collections::HashSet;

fn main() {
    let set_a: HashSet<_> = [1, 2, 3, 4].into_iter().collect();
    let set_b: HashSet<_> = [3, 4, 5, 6].into_iter().collect();

    // 積集合（両方に含まれる要素）
    let intersection: HashSet<_> = set_a.intersection(&set_b).copied().collect();

    println!("A: {set_a:?}");
    println!("B: {set_b:?}");
    println!("A ∩ B: {intersection:?}");
}
