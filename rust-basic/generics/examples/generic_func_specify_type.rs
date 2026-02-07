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
    println!("{}", identity::<i32>(a));
    println!("{}", identity::<i64>(b));
    println!("{}", identity::<f64>(c));

    // make_pair 関数の使用
    println!("{:?}", make_pair::<i32, i64>(a, b));
    println!("{:?}", make_pair::<i64, f64>(b, c));
    println!("{:?}", make_pair::<f64, f64>(c, c));
}
