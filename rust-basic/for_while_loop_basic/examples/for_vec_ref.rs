fn main() {
    let fruits = vec!["apple".to_string(), "banana".to_string(), "blueberry".to_string()];

    // forによる繰り返し (Vec で参照のみ)
    for fruit in &fruits {
        println!("{}", fruit);
    }
}