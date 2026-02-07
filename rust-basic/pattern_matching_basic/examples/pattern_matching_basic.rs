fn main() {
    let value = 2;

    // パターンマッチ
    match value {
        1 => println!("失敗"),
        // ↓ この部分がマッチする
        2 => println!("成功"),
        _ => println!("値は不正値です。"),
    }
}