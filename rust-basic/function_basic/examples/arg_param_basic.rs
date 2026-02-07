// x は仮引数
fn square(x: i32) -> i32 {
    x * x
}

fn main() {
    let x = 2;

    // x は実引数
    let result = square(x);
    
    println!("{} の 2乗 は {} です。", x, result);
}
