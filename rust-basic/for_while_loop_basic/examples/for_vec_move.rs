fn main() {
    let fruits = vec!["apple".to_string(), "banana".to_string(), "blueberry".to_string()];

    // forによる繰り返し (Vec で所有権の移動を伴う)
    for fruit in fruits {
        println!("{}", fruit);
    }

    // 所有権が移動するので以下はエラーとなる
    // println!("{:?}", fruits);
}
