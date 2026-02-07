// 基本的なジェネリック関数
fn identity<T>(x: T) -> T {
    x
}

// 複数の型パラメータを持つ関数
fn make_pair<T, U>(x: T, y: U) -> (T, U){
    (x, y)
}


fn main() {
    let a: i32 = 10;
    let b: i64 = 50;
    let c: f64 = 100.5;

    // identity 関数の使用
    println!("{}", identity(a));
    println!("{}", identity(b));
    println!("{}", identity(c));

    // make_pair 関数の使用
    println!("{:?}", make_pair(a, b));
    println!("{:?}", make_pair(b, c));
    println!("{:?}", make_pair(c, c));
}
