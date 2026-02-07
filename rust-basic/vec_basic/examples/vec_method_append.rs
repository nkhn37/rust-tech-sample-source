fn main() {
    let mut v1 = vec![10, 20, 30];
    let mut v2 = vec![40, 50];

    v1.append(&mut v2);

    println!("{:?}", v1);
    println!("{:?}", v2);
}