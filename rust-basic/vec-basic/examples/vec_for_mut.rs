fn main() {
    let mut v = vec![10, 20, 30, 40, 50];

    // for による繰り返し処理 (可変参照)
    for x in &mut v {
        println!("{}", x);
        *x += 1;
    }
    println!("{:?}", v);
}