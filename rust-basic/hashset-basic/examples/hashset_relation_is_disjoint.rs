use std::collections::HashSet;

fn main() {
    let set_a: HashSet<_> = [1, 2, 3].into_iter().collect();
    let set_b: HashSet<_> = [4, 5, 6].into_iter().collect();
    let set_c: HashSet<_> = [3, 7, 8].into_iter().collect();

    // 共通要素がないかどうかを判定する
    let is_disjoint_ab = set_a.is_disjoint(&set_b);
    let is_disjoint_ac = set_a.is_disjoint(&set_c);
    println!("A と B に共通要素はない : {is_disjoint_ab}");
    println!("A と C に共通要素はない : {is_disjoint_ac}");
}
