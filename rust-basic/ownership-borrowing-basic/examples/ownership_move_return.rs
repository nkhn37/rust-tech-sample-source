fn gives_ownership() -> String {
    let s = String::from("Hello World");
    // 所有権が呼び出し元に移動（ムーブ）する
    s
}

fn main() {
    // 関数で定義した変数の所有権が移動（ムーブ）してくる
    let s1 = gives_ownership();

    println!("{}", s1);
}
