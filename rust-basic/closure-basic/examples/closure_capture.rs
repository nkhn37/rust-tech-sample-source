fn main() {
    // ===== 不変参照でのキャプチャ例
    let x = 10;
    // x は不変参照でキャプチャされる
    let f = |y| x + y;

    // クロージャの呼び出し
    println!("f(5) = {}", f(5));
    // xはまだ使用可能
    println!("x = {}", x);

    // ===== 可変参照でのキャプチャ例
    let mut count = 0;
    println!("count before: {}", count);
    // countは可変参照でキャプチャされる
    let mut g = || count += 1;

    // クロージャを呼び出すとcountが変更される
    g();
    // countは変更されている
    println!("count after: {}", count);

    // ===== move による所有権の移動でのキャプチャ例
    let s = String::from("Hello");
    // s は move で所有権がクロージャに移動される
    let h = move || println!("{s} World!");

    // 実行時に s は消費されて使用できなくなる
    h();
    // s は move で所有権が移動されているため以降は使用できない
    // println!("{s}"); // コンパイルエラー
}
