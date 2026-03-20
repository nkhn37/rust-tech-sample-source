fn main() {
    let mut fruits = vec!["apple".to_string(), "banana".to_string(), "blueberry".to_string()];

    // forによる繰り返し (Vec で変更を伴う)
    for fruit in &mut fruits {
        fruit.push_str("_add");
        println!("{}", fruit);
    }
}
