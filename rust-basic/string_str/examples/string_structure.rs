use std::mem;

fn main() {
    let s1 = String::from("HelloWorld");
    let s2 = String::from("Rust こんにちは！");

    println!("===========================================");
    println!("s1 の文字列の内容: {}", s1);
    println!("s1 のスタック上のアドレス: {:p}", &s1);
    println!("ポインタ: {:p}", s1.as_ptr());
    println!("長さ: {}", s1.len());
    println!("容量: {}", s1.capacity());
    println!("String のサイズ（byte）: {}", mem::size_of_val(&s1));
    println!("===========================================");
    println!("s2 の文字列の内容: {}", s2);
    println!("s2 のスタック上のアドレス: {:p}", &s2);
    println!("ポインタ: {:p}", s2.as_ptr());
    println!("長さ: {}", s2.len());
    println!("容量: {}", s2.capacity());
    println!("String のサイズ（byte）: {}", mem::size_of_val(&s2));
    println!("===========================================");
}