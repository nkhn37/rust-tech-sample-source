fn is_in_range(value: i32, min: i32, max: i32) -> Option<i32> {
    // 値が範囲内に入っているか判定する
    if value >= min && value <= max {
        Some(value)
    } else {
        None
    }
}

fn main() {
    let val = 50;  // 値を変更して試してみてください
    let min = 0;
    let max = 20;

    // Some であれば値を取り出し、None の場合は panic で終了
    // expect によりエラーメッセージを追加
    let result = is_in_range(val, min, max).expect("値は範囲外です。");
    println!("{} は範囲内です。", result);
}