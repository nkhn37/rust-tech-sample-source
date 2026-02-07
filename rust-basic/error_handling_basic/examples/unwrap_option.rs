fn is_in_range(value: i32, min: i32, max: i32) -> Option<i32> {
    // 値が範囲内に入っているか判定する
    if value >= min && value <= max {
        Some(value)
    } else {
        None
    }
}

fn main() {
    let val = 10;  // 値を変更して試してみてください
    let min = 0;
    let max = 20;

    // unwrap で Some であれば値を取り出し、None の場合は panic で終了
    let result = is_in_range(val, min, max).unwrap();
    println!("{} は範囲内です。", result);
}