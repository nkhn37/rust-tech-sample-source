fn main() {
    let mut v1 = vec![10, 20, 30];

    // 要素を指定位置に追加する
    v1.insert(1, 50);
    println!("{:?}", v1);

    // 以下のように範囲外を指定すると panic となる
    // v1.insert(10, 50);
}