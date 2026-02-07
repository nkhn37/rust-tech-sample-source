// 2乗を返す関数 square を定義
fn square(x: i32) -> i32 {
    x * x
}

fn main() {
    // square関数を呼び出し
    let result = square(2);
    
    println!("2 の 2乗 は {} です。", result);
}
