use std::collections::HashSet;

fn main() {
    let all_members: HashSet<_> = ["Alice", "Bob", "Charlie"].into_iter().collect();
    let team_a: HashSet<_> = ["Alice", "Bob"].into_iter().collect();

    // 上位集合かどうかを判定する
    let is_superset_all = all_members.is_superset(&team_a);
    let is_superset_team = team_a.is_superset(&all_members);
    println!("all_members は team_a の上位集合か : {is_superset_all}");
    println!("team_a は all_members の上位集合か : {is_superset_team}");
}
