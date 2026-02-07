// 最小(min), 最大(max) を確認し、(min, max) のタプルで返却する
fn min_max(a: i32, b: i32) -> (i32, i32) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

fn main() {
    let a = 10;
    let b = 5;

    let (min, max) = min_max(a, b);
    println!("min: {}, max: {}", min, max);
}