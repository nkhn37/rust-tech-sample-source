// 呼び出し元の変数を変更する
fn increment(n: &mut i32) {
    *n += 1;
}

fn main() {
    let mut num = 0;

    // 1回目の呼び出し (可変参照)
    increment(&mut num);
    println!("num: {}", num);
    // 2回目の呼び出し (可変参照)
    increment(&mut num);
    println!("num: {}", num);
}
