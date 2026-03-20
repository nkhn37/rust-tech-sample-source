// 所有権が呼び出し元の変数から s に移動（ムーブ）してくる
fn takes_ownership(s: String) {
    println!("{}", s);
}

fn main() {
    let s1 = String::from("Hellow World");

    // 所有権が関数の引数に移動（ムーブ）する
    takes_ownership(s1);

    // ここから s1 は使用できなくなる。
    // println!("{}", s1);
}
