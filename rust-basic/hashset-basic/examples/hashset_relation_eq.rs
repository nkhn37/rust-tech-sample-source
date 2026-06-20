use std::collections::HashSet;

fn main() {
    let set_a: HashSet<_> = ["Alice", "Bob", "Charlie"].into_iter().collect();
    let set_b: HashSet<_> = ["Charlie", "Alice", "Bob"].into_iter().collect();
    let set_c: HashSet<_> = ["Alice", "Bob"].into_iter().collect();

    // 集合が等しいか判定する
    let eq_ab = set_a == set_b;
    let eq_ac = set_a == set_c;
    println!("A と B は等しいか : {eq_ab}");
    println!("A と C は等しいか : {eq_ac}");
}
