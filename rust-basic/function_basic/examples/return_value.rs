fn abs(x: i32) -> i32 {
    if x < 0 {
        // 早期終了 (return)
        return -x;
    }
    // 通常の戻り値
    x
}

fn main() {
    // abs関数を呼び出し
    let result1 = abs(-5);
    let result2 = abs(10);

    println!("結果 result1: {}, result2: {}", result1, result2);

}