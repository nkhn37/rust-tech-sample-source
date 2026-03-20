fn is_in_range(value: i32, min: i32, max: i32) -> Option<i32> {
    // 値が範囲内に入っているか判定する
    if value >= min && value <= max {
        Some(value)
    } else {
        None
    }
}

fn run() -> Option<i32> {
    let val = 10;  // 値を変更して試してみてください
    let min = 0;
    let max = 20;

    // Some が返ってきたら値を取り出し、Noneの場合は即時 return する
    let result = is_in_range(val, min, max)?;
    println!("[run] {} は範囲内です。", result);

    return Some(result);
}

fn main() {

    // 実行と結果の処理
    match run() {
        Some(n) => println!("[main] 成功: {}", n),
        None => println!("[main] エラー: 値は範囲外です。"),
    }
}
