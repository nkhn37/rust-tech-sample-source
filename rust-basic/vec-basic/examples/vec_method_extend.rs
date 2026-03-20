fn main() {
    let mut v1 = vec![10, 20, 30];
    let v2 = vec![40, 50];

    // 所有権を移動しない
    v1.extend(&v2);
    println!("{:?}", v1);
    println!("{:?}", v2);
    
    // 所有権を移動する
    v1.extend(v2);
    println!("{:?}", v1);
    // 所有権が移動するため、以下はコンパイルエラーになる
    // println!("{:?}", v2);

    // 配列でも追加できる
    let arr = [60, 70, 80];
    v1.extend(arr);
    println!("{:?}", v1);
}