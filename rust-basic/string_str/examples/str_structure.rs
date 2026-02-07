use std::mem;

fn main() {
    let s1 = "HelloWorld";
    let s2 = String::from("Rust こんにちは！");
    let s2_slice = &s2[..];

    println!("===========================================");
    println!("s1 の文字列の内容: {}", s1);
    println!("s1 のスタック上のアドレス: {:p}", &s1);
    println!("ポインタ: {:p}", s1.as_ptr());
    println!("長さ: {}", s1.len());
    println!("&str のサイズ（byte）: {}", mem::size_of_val(&s1));
    println!("===========================================");
    println!("s2_slice の文字列の内容: {}", s2_slice);
    println!("s2_slice のスタック上のアドレス: {:p}", &s2_slice);
    println!("ポインタ: {:p}", s2_slice.as_ptr());
    println!("長さ: {}", s2_slice.len());
    println!("&str のサイズ（byte）: {}", mem::size_of_val(&s2_slice));
    println!("===========================================");
}
