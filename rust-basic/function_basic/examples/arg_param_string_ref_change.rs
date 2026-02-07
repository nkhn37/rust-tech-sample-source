// 不変参照 (読み取り専用) の場合
fn print_str_ref(s: &String) {
    println!("print_str_ref: {}", s);
}

// 可変参照 (変更可) の場合
fn print_str_change(s: &mut String) {
    s.push_str("!!!");
    println!("print_str_change: {}", s);
}

fn main() {
    // 不変参照の場合
    let mut s1 = String::from("World World");
    print_str_ref(&s1);

    // 可変参照の場合
    print_str_change(&mut s1);
    println!("呼び出し元のs1: {}", s1);
}