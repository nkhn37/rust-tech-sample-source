// クロージャも引数として受け取ることができる関数
fn call_function_closure<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(x)
}

// 1を加算する通常の関数
fn add_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    // クロージャで使用する外部変数
    let base = 1;

    // クロージャーを定義
    let add_one_closure = |x| x + base;
    println!("クロージャーを呼び出す: {}", add_one_closure(5));

    // 通常の関数を呼び出す
    println!(
        "通常の関数を call_function 経由で呼び出す: {}",
        call_function_closure(add_one, 5)
    );

    // クロージャーを call_function_closure に渡す
    println!(
        "クロージャーを call_function_closure 経由で呼び出す: {}",
        call_function_closure(add_one_closure, 5)
    );
}
