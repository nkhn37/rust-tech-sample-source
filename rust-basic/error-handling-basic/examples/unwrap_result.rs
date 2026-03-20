fn double_if_in_range(value: i32, min: i32, max: i32) -> Result<i32, String> {
    // 値が範囲に入っていたら 2 倍する
    if value >= min && value <= max {
        Ok(2 * value)
    } else {
        Err(String::from("値は範囲外です。"))
    }
}

fn main() {
    let val = 10;  // 値を変更して試してみてください
    let min = 0;
    let max = 20;

    // unwrap で Ok であれば値を取り出し、None の場合は panic で終了
    let result = double_if_in_range(val, min, max).unwrap();
    println!("計算結果は {} です。", result);
}