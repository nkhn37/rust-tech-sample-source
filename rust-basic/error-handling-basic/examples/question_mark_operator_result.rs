fn double_if_in_range(value: i32, min: i32, max: i32) -> Result<i32, String> {
    // 値が範囲に入っていたら 2 倍する
    if value >= min && value <= max {
        Ok(2 * value)
    } else {
        Err(String::from("値は範囲外です。"))
    }
}

fn run() -> Result<i32, String> {
    let val = 10;  // 値を変更して試してみてください
    let min = 0;
    let max = 20;

    // Ok が返ってきたら値を取り出し、Error の場合は即時 return する
    let result = double_if_in_range(val, min, max)?;
    println!("[run] 計算結果: {}", result);

    // 成功の場合は、Ok を返却
    Ok(result)
}

fn main() {
    // 実行と結果の処理
    match run() {
        Ok(_) => println!("[main] 成功"),
        Err(e) => println!("[main] 失敗: {}", e),
    }
}
