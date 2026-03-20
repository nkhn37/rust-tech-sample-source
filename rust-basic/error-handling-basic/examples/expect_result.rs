fn double_if_in_range(value: i32, min: i32, max: i32) -> Result<i32, String> {
    // 値が範囲に入っていたら 2 倍する
    if value >= min && value <= max {
        Ok(2 * value)
    } else {
        Err(String::from("値は範囲外です。"))
    }
}

fn main() {
    let val = 50;  // 値を変更して試してみてください
    let min = 0;
    let max = 20;

    // Ok であれば値を取り出し、None の場合は panic で終了
    // expect によりエラーメッセージを追加
    let result = double_if_in_range(val, min, max).expect("エラー発生");
    println!("計算結果は {} です。", result);
}