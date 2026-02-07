use std::error::Error;

fn is_in_range(value: i32, min: i32, max: i32) -> Option<i32> {
    // 値が範囲内に入っているか判定する
    if value >= min && value <= max {
        Some(value)
    } else {
        None
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let val = 10;  // 値を変更して試してみてください
    let min = 0;
    let max = 20;

    // Some が返ってきたら値を取り出し、Noneの場合は Resultへ変換 する
    let result = is_in_range(val, min, max).ok_or("値は範囲外です。")?;
    println!("{} は範囲内です。", result);

    Ok(())
}
