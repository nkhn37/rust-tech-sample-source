fn main() {
    let mut count = 0;

    // loopによる無限ループ (breakが必要)
    loop {
        // 5以上の場合、ループを終了する
        if count >= 5 {
            break;
        }

        println!("count = {}", count);
        count += 1;
    }
}