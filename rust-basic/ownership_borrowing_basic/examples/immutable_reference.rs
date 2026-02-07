fn print_message(s: &str) {
    // 不変参照なので読み取りのみ
    println!("{}", s);
}

fn main() {
    let s1 = String::from("Hello World");

    // s1 の内容を不変参照で関数に渡す
    print_message(&s1);

    // s1 は所有権を持っているので、ここでもまだ使える
    println!("{}", s1);
}
