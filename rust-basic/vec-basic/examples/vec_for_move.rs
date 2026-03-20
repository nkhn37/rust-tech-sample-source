fn main() {
    let v = vec![10, 20, 30, 40, 50];

    // 所有権を移動する
    for mut x in v {
        x += 1;
        println!("{}", x);
    }

    // 所有権が移動しているのでコンパイルエラーになる。 
    // println!("{:?}", v);
}