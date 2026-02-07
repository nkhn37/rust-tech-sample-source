fn main() {
    let mut v = vec![10, 20, 30, 40, 50];

    // 末尾の要素を削除する
    let last = v.pop();
    println!("{:?}", v);
    println!("{:?}", last);
}