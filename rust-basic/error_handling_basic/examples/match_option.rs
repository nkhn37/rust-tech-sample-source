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

    // match で Option 型の返却値を処理する
    match is_in_range(val, min, max) {
        Some(n) => println!("{} は範囲内です。", n),
        None => println!("値は範囲外です。"),
    }
}