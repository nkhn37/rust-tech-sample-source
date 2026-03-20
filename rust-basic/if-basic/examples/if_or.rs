fn main() {
    let is_member = true;
    let has_coupon = false;

    // 論理和 (OR) で条件を指定する
    if is_member || has_coupon {
        println!("割引が適用されます。")
    } else {
        println!("割引は適用されません。")
    }
}
