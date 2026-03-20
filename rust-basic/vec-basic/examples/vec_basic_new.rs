fn main() {
    // 関連関数 new で Vec を生成する
    let mut v: Vec<i32> = Vec::new();

    // 要素を追加する
    v.push(1);
    v.push(2);
    v.push(3);

    println!("{:?}", v);
}