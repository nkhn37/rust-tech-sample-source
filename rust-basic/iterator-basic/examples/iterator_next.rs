fn main() {
    let v = vec![1, 2, 3];
    let mut iter = v.iter();

    println!("{:?}", iter.next()); // Some(1)
    println!("{:?}", iter.next()); // Some(2)
    println!("{:?}", iter.next()); // Some(3)
    println!("{:?}", iter.next()); // None
}
