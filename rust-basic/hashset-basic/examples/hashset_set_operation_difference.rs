use std::collections::HashSet;

fn main() {
    let set_a: HashSet<_> = [1, 2, 3, 4].into_iter().collect();
    let set_b: HashSet<_> = [3, 4, 5].into_iter().collect();

    // 差集合（A にだけ含まれる要素）
    let difference_a_b: HashSet<_> = set_a.difference(&set_b).copied().collect();

    // 差集合（B にだけ含まれる要素）
    let difference_b_a: HashSet<_> = set_b.difference(&set_a).copied().collect();

    println!("A: {set_a:?}");
    println!("B: {set_b:?}");
    println!("A - B: {difference_a_b:?}");
    println!("B - A: {difference_b_a:?}");
}
