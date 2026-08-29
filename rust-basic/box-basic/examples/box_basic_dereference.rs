fn main() {
    let b = Box::new(5);

    // * で参照外しを行い、Box の中身の値を取り出して計算に使用する
    let result = *b + 10;

    // Box は、Display トレイトを実装しているため、println マクロで参照外しが不要
    println!("b : {b}");
    println!("result : {result}");
}
