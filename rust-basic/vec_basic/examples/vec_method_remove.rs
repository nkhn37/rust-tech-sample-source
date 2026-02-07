fn main() {
    let mut v = vec![10, 20, 30, 40, 50];

    // 指定位置を削除
    let removed = v.remove(1);
    println!("{:?}", v);
    println!("{}", removed);

    // 以下のように範囲外を指定すると panic となる
    // let removed = v.remove(10);
}
