fn main() {
    let v = vec![10, 20, 30, 40, 50];

    // for による繰り返し処理 (不変参照)
    for x in v.iter() {
        println!("{}", x);
    }
}