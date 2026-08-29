fn main() {
    // new で i32 型の値をヒープ領域に確保する
    let b = Box::new(5);
    println!("b : {b}");
}
