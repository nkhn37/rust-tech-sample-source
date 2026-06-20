use std::collections::HashSet;

fn main() {
    let all_members: HashSet<_> = ["Alice", "Bob", "Charlie"].into_iter().collect();
    let team_a: HashSet<_> = ["Alice", "Bob"].into_iter().collect();

    // 部分集合かどうかを判定する
    let is_subset_team = team_a.is_subset(&all_members);
    let is_subset_all = all_members.is_subset(&team_a);
    println!("team_a は all_members の部分集合か : {is_subset_team}");
    println!("all_members は team_a の部分集合か : {is_subset_all}");
}
