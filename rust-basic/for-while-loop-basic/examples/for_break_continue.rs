fn main() {
    // break, continue による繰り返しの制御
    for i in 0..10 {
        if i == 2 {
            continue;  // 後続の処理をスキップ
        }
        if i == 5 {
            break; // ループを終了して抜ける
        }
        println!("i = {}", i);
    }
}