fn main () {
    let v = vec![10, 20, 30];

    println!("{}", v[0]);
    println!("{:?}", v.get(2));
    println!("{:?}", v.get(100));

    // 以下のように範囲外を指定すると panic となる
    // println!("{}", v[100]); 
}
