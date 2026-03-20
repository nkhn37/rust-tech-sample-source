fn main() {
    // コピーされる例 (i32)
    let x = 10;
    let y = x;
    // コピーなので x も y も使用できる
    println!("{}", x);
    println!("{}", y);

    // 移動 (ムーブ) される例 (String)
    let s1 = String::from("Hello World");
    let s2 = s1;
    // 所有権が移っているので s2 は使える
    println!("{}", s2);
    // s1 は使えない (コメントアウトを外すとコンパイルエラー)
    // println!("{}", s1);
}