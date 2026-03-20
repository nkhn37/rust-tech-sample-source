fn main() {
    // クロージャで使用する外部変数
    let base = 1;

    // クロージャーを定義
    let add_one_closure = |x| x + base;
    println!("クロージャーを呼び出す: {}", add_one_closure(5));
}
