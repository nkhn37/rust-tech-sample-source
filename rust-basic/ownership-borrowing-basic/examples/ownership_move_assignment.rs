fn main() {
    let s1 = String::from("Hello World!");

    // 所有権が s1 から s2 に移動（ムーブ）する
    let s2 = s1;

    // ここから s1 は使用できなくなる
    println!("{}", s1);
}
