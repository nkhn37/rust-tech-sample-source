use std::arch::x86_64;

fn main() {
    let v1 = vec![1, 2, 3];

    // 不変参照のイテレータを生成
    for x in (&v1).into_iter() {
        println!("{}", x);
    }

    let mut v2 = vec![4, 5, 6];

    // 可変参照のイテレータを生成
    for x in (&mut v2).into_iter() {
        // 各要素に1を加える
        *x += 1;
        println!("{}", x);
    }

    let v3 = vec![7, 8, 9];

    // 所有権を移動するイテレータを生成
    for x in v3.into_iter() {
        println!("{}", x);
    }

    // v3は所有権が移動したため、ここで使用できない
    // コンパイルエラーとなる
    // println!("{:?}", v3);
}
