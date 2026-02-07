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

    // match で Result 型の返却値を処理する
    match double_if_in_range(val, min, max) {
        Ok(n) => println!("計算結果は {} です。", n),
        Err(e) => println!("Error: {}", e),
    }
}