fn main() {
    let v1 = vec![10, 20, 30];
    let v2 = vec![40, 50];
    let v3 = vec![60, 70, 80, 90, 100];

    let v_concat = vec![v1, v2, v3].concat();
    println!("{:?}", v_concat);

    // 所有権は移動しているので以下はコンパイルエラーになる
    // println!("{:?}", v1);
    // println!("{:?}", v2);
    // println!("{:?}", v3);
}