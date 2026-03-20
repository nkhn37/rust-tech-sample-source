fn main() {
    let x = 5;
    println!("xの値: {}", x);
    {
        // 異なるスコープでシャドーイングする
        let x = 10;
        println!("xの値: {}", x);
    }
    println!("xの値: {}", x);
}
