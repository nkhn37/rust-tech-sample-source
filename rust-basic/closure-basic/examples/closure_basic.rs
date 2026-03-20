fn main() {
    // ===== 単一の式のクロージャ例
    let add = |a, b| a + b;
    println!("add(2, 3) = {}", add(2, 3));

    // ===== 引数なしのクロージャ例
    let greet = || "Hello, Rust!";
    println!("greet() = {}", greet());

    // ===== ブロック形式のクロージャ例
    let multiply = |a, b| {
        println!("calculating {a} * {b} ...");
        // 最後の式がクロージャの戻り値になる
        a * b
    };
    println!("multiply(3, 5) = {}", multiply(3, 5));

    // ===== 外側の変数をキャプチャするクロージャ例
    let base = 10;

    let add_to_base = |x| base + x;
    println!("add_to_base(5) = {}", add_to_base(5));

    // ===== 単一の式のクロージャ例 (型注釈あり)
    let subtract = |a: i32, b: i32| a - b;
    println!("subtract(5, 2) = {}", subtract(5, 2));
}
